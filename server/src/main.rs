// server/src/main.rs
use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Cell, Gauge, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
        Wrap, Clear,
    },
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    io,
    process::{Child, Command},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, mpsc, RwLock, Mutex as TokioMutex},
    time::interval,
};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use fv_plugin::{discover_plugins, LoadedPlugin};
use service_registry::LocalServiceRegistry;
mod mesh;
use finalverse_server::{
    ServiceInfo, ServiceStatus, LogEntry, LogLevel, ServerCommand, ServerResponse,
};
use tonic_health::server::{health_reporter, HealthServer, Health};
use tonic::transport::Server as GrpcServer;

#[derive(Parser)]
#[command(name = "finalverse-server")]
#[command(about = "Finalverse Server Management Console")]
struct Args {
    #[arg(short, long, default_value = "8090")]
    port: u16,
    
    #[arg(short, long, default_value = "info")]
    log_level: String,
    
    #[arg(long)]
    headless: bool,
}


pub struct ServerManager {
    services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
    processes: Arc<Mutex<HashMap<String, Child>>>,
    log_buffer: Arc<RwLock<VecDeque<LogEntry>>>,
    command_tx: mpsc::UnboundedSender<ServerCommand>,
    command_rx: Arc<TokioMutex<mpsc::UnboundedReceiver<ServerCommand>>>,
    broadcast_tx: broadcast::Sender<ServerResponse>,
}

impl ServerManager {
    pub fn new() -> Self {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (broadcast_tx, _) = broadcast::channel(100);
        
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            processes: Arc::new(Mutex::new(HashMap::new())),
            log_buffer: Arc::new(RwLock::new(VecDeque::with_capacity(10000))),
            command_tx,
            command_rx: Arc::new(TokioMutex::new(command_rx)),
            broadcast_tx,
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        // Initialize service definitions
        let services = vec![
            ("websocket-gateway", 3000),
            ("api-gateway", 8080),
            ("ai-orchestra", 3004),
            ("song-engine", 3001),
            ("story-engine", 3005),
            ("echo-engine", 3003),
            ("world-engine", 3002),
            ("harmony-service", 3006),
            ("asset-service", 3007),
            ("community", 3008),
            ("silence-service", 3009),
            ("procedural-gen", 3010),
            ("behavior-ai", 3011),
        ];

        let mut service_map = self.services.write().await;
        for (name, port) in services {
            service_map.insert(
                name.to_string(),
                ServiceInfo {
                    name: name.to_string(),
                    port,
                    status: ServiceStatus::Stopped,
                    pid: None,
                    uptime: Duration::from_secs(0),
                    last_health_check: None,
                    health_status: false,
                    cpu_usage: 0.0,
                    memory_usage: 0,
                    log_lines: VecDeque::with_capacity(1000),
                },
            );
        }

        Ok(())
    }

    pub async fn start_service(&self, name: &str) -> Result<()> {
        let binary_path = format!("target/release/{}", name);
        
        if !std::path::Path::new(&binary_path).exists() {
            return Err(anyhow::anyhow!("Binary not found: {}", binary_path));
        }

        // Update status to starting
        {
            let mut services = self.services.write().await;
            if let Some(service) = services.get_mut(name) {
                service.status = ServiceStatus::Starting;
            }
        }

        // Start the process
        let mut cmd = Command::new(&binary_path);
        cmd.env("RUST_LOG", "info");
        
        match cmd.spawn() {
            Ok(child) => {
                let pid = child.id();
                
                // Store the process
                {
                    let mut processes = self.processes.lock().unwrap();
                    processes.insert(name.to_string(), child);
                }

                // Update service status
                {
                    let mut services = self.services.write().await;
                    if let Some(service) = services.get_mut(name) {
                        service.status = ServiceStatus::Running;
                        service.pid = Some(pid);
                    }
                }

                self.log_event(name, LogLevel::Info, &format!("Service started with PID {}", pid)).await;
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("Failed to start service: {}", e);
                
                // Update status to error
                {
                    let mut services = self.services.write().await;
                    if let Some(service) = services.get_mut(name) {
                        service.status = ServiceStatus::Error(error_msg.clone());
                    }
                }

                self.log_event(name, LogLevel::Error, &error_msg).await;
                Err(anyhow::anyhow!(error_msg))
            }
        }
    }

    pub async fn stop_service(&self, name: &str) -> Result<()> {
        // Update status to stopping
        {
            let mut services = self.services.write().await;
            if let Some(service) = services.get_mut(name) {
                service.status = ServiceStatus::Stopping;
            }
        }

        // Stop the process
        {
            let mut processes = self.processes.lock().unwrap();
            if let Some(mut child) = processes.remove(name) {
                match child.kill() {
                    Ok(_) => {
                        self.log_event(name, LogLevel::Info, "Service stopped").await;
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to stop service: {}", e);
                        self.log_event(name, LogLevel::Error, &error_msg).await;
                        return Err(anyhow::anyhow!(error_msg));
                    }
                }
            }
        }

        // Update service status
        {
            let mut services = self.services.write().await;
            if let Some(service) = services.get_mut(name) {
                service.status = ServiceStatus::Stopped;
                service.pid = None;
                service.uptime = Duration::from_secs(0);
            }
        }

        Ok(())
    }

    pub async fn restart_service(&self, name: &str) -> Result<()> {
        self.stop_service(name).await?;
        tokio::time::sleep(Duration::from_secs(2)).await;
        self.start_service(name).await
    }

    async fn log_event(&self, service: &str, level: LogLevel, message: &str) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level,
            service: service.to_string(),
            message: message.to_string(),
        };

        // Add to global log buffer
        {
            let mut log_buffer = self.log_buffer.write().await;
            log_buffer.push_back(entry.clone());
            if log_buffer.len() > 10000 {
                log_buffer.pop_front();
            }
        }

        // Add to service-specific log buffer
        {
            let mut services = self.services.write().await;
            if let Some(service_info) = services.get_mut(service) {
                service_info.log_lines.push_back(entry.clone());
                if service_info.log_lines.len() > 1000 {
                    service_info.log_lines.pop_front();
                }
            }
        }

        // Broadcast to clients
        let _ = self.broadcast_tx.send(ServerResponse::Logs(vec![entry]));
    }

    pub async fn run_command_handler(&self) {
        let command_rx = self.command_rx.clone();
        let services = self.services.clone();
        let broadcast_tx = self.broadcast_tx.clone();

        tokio::spawn(async move {
            let mut rx = command_rx.lock().await;
            while let Some(command) = rx.recv().await {
                match command {
                    ServerCommand::StartService(name) => {
                        // Handle start service
                    }
                    ServerCommand::StopService(name) => {
                        // Handle stop service
                    }
                    ServerCommand::GetAllServices => {
                        let services_vec: Vec<ServiceInfo> = {
                            let services_guard = services.read().await;
                            services_guard.values().cloned().collect()
                        };
                        let _ = broadcast_tx.send(ServerResponse::AllServices(services_vec));
                    }
                    _ => {}
                }
            }
        });
    }

    pub async fn run_health_monitor(&self) {
        let services = self.services.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                let services_to_check: Vec<String> = {
                    let services_guard = services.read().await;
                    services_guard.keys().cloned().collect()
                };

                for service_name in services_to_check {
                    let port = {
                        let services_guard = services.read().await;
                        if let Some(service) = services_guard.get(&service_name) {
                            service.port
                        } else {
                            continue;
                        }
                    };

                    // Check health endpoint
                    let health_url = format!("http://localhost:{}/health", port);
                    let is_healthy = match reqwest::get(&health_url).await {
                        Ok(response) => response.status().is_success(),
                        Err(_) => false,
                    };

                    // Update health status
                    {
                        let mut services_guard = services.write().await;
                        if let Some(service) = services_guard.get_mut(&service_name) {
                            service.health_status = is_healthy;
                            service.last_health_check = Some(Utc::now());
                        }
                    }
                }
            }
        });
    }
}

// TUI Application State
pub struct App {
    server_manager: Arc<ServerManager>,
    current_tab: usize,
    services_list_state: ListState,
    log_scroll: usize,
    command_input: String,
    command_history: Vec<String>,
    show_help: bool,
}

impl App {
    pub fn new(server_manager: Arc<ServerManager>) -> Self {
        let mut services_list_state = ListState::default();
        services_list_state.select(Some(0));

        Self {
            server_manager,
            current_tab: 0,
            services_list_state,
            log_scroll: 0,
            command_input: String::new(),
            command_history: Vec::new(),
            show_help: false,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_app(&mut terminal).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        result
    }

    async fn run_app<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(250);

        loop {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if self.handle_input(key).await? {
                        break;
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
            .split(f.size());

        self.render_tabs(f, chunks[0]);
        self.render_main_content(f, chunks[1]);
        self.render_command_bar(f, chunks[2]);

        if self.show_help {
            self.render_help_popup(f);
        }
    }

    fn render_tabs(&self, f: &mut Frame, area: Rect) {
        let titles = vec!["Services", "Logs", "Metrics", "Commands"];
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Finalverse Server Console"))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(self.current_tab);
        f.render_widget(tabs, area);
    }

    fn render_main_content(&mut self, f: &mut Frame, area: Rect) {
        match self.current_tab {
            0 => self.render_services_tab(f, area),
            1 => self.render_logs_tab(f, area),
            2 => self.render_metrics_tab(f, area),
            3 => self.render_commands_tab(f, area),
            _ => {}
        }
    }

    fn render_services_tab(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Services list
        let services: Vec<ListItem> = vec![
            ListItem::new("🌐 websocket-gateway [RUNNING]"),
            ListItem::new("🚪 api-gateway [RUNNING]"),
            ListItem::new("🎵 song-engine [RUNNING]"),
            ListItem::new("🔮 echo-engine [STOPPED]"),
            ListItem::new("🌍 world-engine [ERROR]"),
        ];

        let services_list = List::new(services)
            .block(Block::default().borders(Borders::ALL).title("Services"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">> ");

        f.render_stateful_widget(services_list, chunks[0], &mut self.services_list_state);

        // Service details
        let service_info = Paragraph::new(
            "Service: websocket-gateway\n\
             Status: Running\n\
             PID: 12345\n\
             Port: 3000\n\
             Uptime: 2h 15m\n\
             CPU: 2.3%\n\
             Memory: 45.2 MB\n\
             Last Health Check: 2024-01-15 14:30:25\n\
             Health Status: OK"
        )
        .block(Block::default().borders(Borders::ALL).title("Service Details"))
        .wrap(Wrap { trim: true });

        f.render_widget(service_info, chunks[1]);
    }

    fn render_logs_tab(&self, f: &mut Frame, area: Rect) {
        let logs = vec![
            Line::from(vec![
                Span::styled("2024-01-15 14:30:15", Style::default().fg(Color::Gray)),
                Span::raw(" "),
                Span::styled("INFO", Style::default().fg(Color::Green)),
                Span::raw(" "),
                Span::styled("websocket-gateway", Style::default().fg(Color::Cyan)),
                Span::raw(" Connection established"),
            ]),
            Line::from(vec![
                Span::styled("2024-01-15 14:30:16", Style::default().fg(Color::Gray)),
                Span::raw(" "),
                Span::styled("ERROR", Style::default().fg(Color::Red)),
                Span::raw(" "),
                Span::styled("song-engine", Style::default().fg(Color::Cyan)),
                Span::raw(" Failed to connect to database"),
            ]),
        ];

        let logs_paragraph = Paragraph::new(logs)
            .block(Block::default().borders(Borders::ALL).title("System Logs"))
            .wrap(Wrap { trim: true });

        f.render_widget(logs_paragraph, area);
    }

    fn render_metrics_tab(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // System metrics
        let cpu_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("CPU Usage"))
            .gauge_style(Style::default().fg(Color::Yellow))
            .percent(45);

        f.render_widget(cpu_gauge, chunks[0]);

        // Memory gauge
        let memory_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Memory Usage"))
            .gauge_style(Style::default().fg(Color::Blue))
            .percent(67);

        f.render_widget(memory_gauge, chunks[1]);
    }

    fn render_commands_tab(&self, f: &mut Frame, area: Rect) {
        let help_text = 
            "Available Commands:\n\n\
             start <service>     - Start a service\n\
             stop <service>      - Stop a service\n\
             restart <service>   - Restart a service\n\
             status [service]    - Show service status\n\
             logs <service> [n]  - Show service logs\n\
             health              - Run health check\n\
             shutdown            - Shutdown server\n\
             help                - Show this help\n\n\
             Navigation:\n\
             Tab/Shift+Tab       - Switch tabs\n\
             ↑/↓                 - Navigate lists\n\
             Enter               - Execute command\n\
             Esc                 - Cancel/Go back\n\
             Ctrl+C              - Quit";

        let help_paragraph = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title("Commands & Help"))
            .wrap(Wrap { trim: true });

        f.render_widget(help_paragraph, area);
    }

    fn render_command_bar(&self, f: &mut Frame, area: Rect) {
        let input = Paragraph::new(format!("> {}", self.command_input))
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Command"));
        f.render_widget(input, area);
    }

    fn render_help_popup(&self, f: &mut Frame) {
        let area = self.centered_rect(60, 20, f.size());
        f.render_widget(Clear, area);
        
        let help_text = "Finalverse Server Console Help\n\nPress '?' to toggle this help\nPress 'q' to quit";
        let help_block = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title("Help"))
            .wrap(Wrap { trim: true });
        f.render_widget(help_block, area);
    }

    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }

    async fn handle_input(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('?') => self.show_help = !self.show_help,
            KeyCode::Tab => self.next_tab(),
            KeyCode::BackTab => self.previous_tab(),
            KeyCode::Up => self.previous_service(),
            KeyCode::Down => self.next_service(),
            KeyCode::Enter => self.execute_command().await?,
            KeyCode::Char(c) => self.command_input.push(c),
            KeyCode::Backspace => { self.command_input.pop(); },
            _ => {}
        }
        Ok(false)
    }

    fn next_tab(&mut self) {
        self.current_tab = (self.current_tab + 1) % 4;
    }

    fn previous_tab(&mut self) {
        if self.current_tab > 0 {
            self.current_tab -= 1;
        } else {
            self.current_tab = 3;
        }
    }

    fn next_service(&mut self) {
        let i = match self.services_list_state.selected() {
            Some(i) => (i + 1) % 5, // Assuming 5 services for now
            None => 0,
        };
        self.services_list_state.select(Some(i));
    }

    fn previous_service(&mut self) {
        let i = match self.services_list_state.selected() {
            Some(i) => if i == 0 { 4 } else { i - 1 },
            None => 0,
        };
        self.services_list_state.select(Some(i));
    }

    async fn execute_command(&mut self) -> Result<()> {
        if self.command_input.is_empty() {
            return Ok(());
        }

        let command = self.command_input.clone();
        self.command_history.push(command.clone());
        self.command_input.clear();

        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        match parts[0] {
            "start" => {
                if parts.len() > 1 {
                    self.server_manager.start_service(parts[1]).await?;
                }
            }
            "stop" => {
                if parts.len() > 1 {
                    self.server_manager.stop_service(parts[1]).await?;
                }
            }
            "restart" => {
                if parts.len() > 1 {
                    self.server_manager.restart_service(parts[1]).await?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    env_logger::init();

    // Create server manager
    let server_manager = Arc::new(ServerManager::new());
    server_manager.initialize().await?;

    // Service registry and dynamic plugins
    let registry = LocalServiceRegistry::new();
    let mut plugins = discover_plugins().await;
    for p in &plugins {
        p.instance.init(&registry).await?;
    }

    mesh::spawn_refresh_task();

    // gRPC server aggregating plugin services
    let grpc_port: u16 = std::env::var("FINALVERSE_GRPC_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(50051);
    let grpc_plugins = plugins;
    let grpc_addr = format!("0.0.0.0:{}", grpc_port).parse()?;

    tokio::spawn(async move {
        // Build the gRPC server with all plugin services
        let (_health_reporter, health_service) = health_reporter();
        let mut router = GrpcServer::builder().add_service(health_service);

        // Register each plugin's gRPC services
        for mut plugin in grpc_plugins {
            let instance = plugin.take_instance();
            router = instance.register_grpc(router);
        }

        println!("🚀 Starting gRPC server on {}", grpc_addr);

        if let Err(e) = router.serve(grpc_addr).await {
            eprintln!("❌ gRPC server error: {}", e);
        }
    });

    // Start background tasks
    server_manager.run_command_handler().await;
    server_manager.run_health_monitor().await;

    // WebSocket server for CLI connections
    let ws_port = args.port;
    let ws_manager = Arc::clone(&server_manager);
    tokio::spawn(async move {
        if let Err(e) = run_websocket_server(ws_port, ws_manager).await {
            eprintln!("❌ WebSocket server error: {}", e);
        }
    });

    if args.headless {
        // Run in headless mode (no TUI)
        println!("🎵 Finalverse Server starting in headless mode on port {}", args.port);
        futures::future::pending::<()>().await;
    } else {
        // Run with TUI
        println!("🎵 Starting Finalverse Server Console...");

        let mut app = App::new(Arc::clone(&server_manager));
        app.run().await?;
    }

    Ok(())
}

async fn handle_client(stream: TcpStream, server_manager: Arc<ServerManager>) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    println!("📱 New CLI client connected");

    // Handle WebSocket messages for CLI communication
    // This would implement the protocol for CLI commands

    Ok(())
}

async fn run_websocket_server(port: u16, server_manager: Arc<ServerManager>) -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    println!("📡 WebSocket server listening on ws://127.0.0.1:{port}");

    loop {
        let (stream, _) = listener.accept().await?;
        let manager = Arc::clone(&server_manager);
        tokio::spawn(handle_client(stream, manager));
    }
}
