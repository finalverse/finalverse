// finalverse-cli/src/main.rs
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use rustyline::{error::ReadlineError, DefaultEditor};
use serde_json;
use std::collections::HashMap;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::SplitSink;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
use tokio::net::TcpStream;

use finalverse_server::{ServerCommand, ServerResponse, ServiceInfo, LogEntry};

#[derive(Parser)]
#[command(name = "finalverse-cli")]
#[command(about = "Finalverse CLI - Remote management for Finalverse Server")]
#[command(version = "1.0")]
struct Cli {
    #[arg(short, long, default_value = "ws://127.0.0.1:8090")]
    server: String,

    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    interactive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a service
    Start {
        /// Service name to start
        service: String,
    },
    /// Stop a service
    Stop {
        /// Service name to stop
        service: String,
    },
    /// Restart a service
    Restart {
        /// Service name to restart
        service: String,
    },
    /// Show service status
    Status {
        /// Optional service name (shows all if not specified)
        service: Option<String>,
    },
    /// Show service logs
    Logs {
        /// Service name
        service: Option<String>,
        /// Number of lines to show
        #[arg(short, long, default_value = "50")]
        lines: usize,
        /// Follow logs in real-time
        #[arg(short, long)]
        follow: bool,
    },
    /// Run health check on all services
    Health,
    /// Execute a custom command
    Exec {
        /// Command to execute
        command: String,
    },
    /// Start conversational chat mode
    Chat,
    /// Start interactive mode
    Interactive,
    /// Shutdown the server
    Shutdown,
}

pub struct FinalverseCli {
    server_url: String,
    ws: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>, 
}

impl FinalverseCli {
    pub fn new(server_url: String) -> Self {
        Self {
            server_url,
            ws: None,
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        println!("Connecting to {}...", self.server_url);

        let (ws_stream, _) = connect_async(&self.server_url).await
            .context("Failed to connect to server")?;

        let (write, read) = ws_stream.split();
        self.ws = Some(write);

        // Spawn a task to handle incoming messages
        tokio::spawn(async move {
            read.for_each(|message| async {
                match message {
                    Ok(msg) => {
                        if let Ok(text) = msg.to_text() {
                            println!("Server: {}", text);
                        }
                    }
                    Err(e) => eprintln!("Error receiving message: {}", e),
                }
            }).await;
        });

        println!("Connected successfully!");
        Ok(())
    }

    pub async fn send_command(&mut self, command: &str) -> Result<()> {
        if let Some(ws) = &mut self.ws {
            ws.send(Message::Text(command.to_string())).await
                .context("Failed to send command")?;
        } else {
            return Err(anyhow::anyhow!("Not connected to server"));
        }
        Ok(())
    }

    pub async fn query_world_state(&mut self) -> Result<()> {
        let command = serde_json::json!({
            "type": "query",
            "target": "world_state"
        });
        self.send_command(&command.to_string()).await
    }

    pub async fn query_harmony_levels(&mut self) -> Result<()> {
        let command = serde_json::json!({
            "type": "query",
            "target": "harmony_levels"
        });
        self.send_command(&command.to_string()).await
    }

    pub async fn create_npc(&mut self, name: String, location: String) -> Result<()> {
        let command = serde_json::json!({
            "type": "create",
            "entity": "npc",
            "data": {
                "name": name,
                "location": location
            }
        });
        self.send_command(&command.to_string()).await
    }

    pub async fn generate_quest(&mut self, quest_type: String, difficulty: u32) -> Result<()> {
        let command = serde_json::json!({
            "type": "generate",
            "entity": "quest",
            "data": {
                "type": quest_type,
                "difficulty": difficulty
            }
        });
        self.send_command(&command.to_string()).await
    }

    pub async fn trigger_event(&mut self, event_type: String, params: serde_json::Value) -> Result<()> {
        let command = serde_json::json!({
            "type": "trigger",
            "event": event_type,
            "params": params
        });
        self.send_command(&command.to_string()).await
    }

    pub async fn chat_mode(&mut self) -> Result<()> {
        println!("Entering AI chat mode. Type 'exit' to quit.");
        let mut rl = DefaultEditor::new()?;

        loop {
            match rl.readline("you> ") {
                Ok(line) => {
                    let line = line.trim();
                    if line.eq_ignore_ascii_case("exit") {
                        break;
                    }
                    if line.is_empty() {
                        continue;
                    }
                    rl.add_history_entry(line)?;
                    let cmd = serde_json::json!({
                        "type": "chat",
                        "message": line,
                    });
                    self.send_command(&cmd.to_string()).await?;
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        Ok(())
    }

    pub async fn interactive_mode(&mut self) -> Result<()> {
        println!("Entering interactive mode. Type 'help' for commands, 'exit' to quit.");

        let mut rl = DefaultEditor::new()?;

        loop {
            let readline = rl.readline("finalverse> ");
            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    rl.add_history_entry(&*line)?;

                    let parts: Vec<&str> = line.split_whitespace().collect();
                    match parts.get(0) {
                        Some(&"exit") | Some(&"quit") => break,
                        Some(&"help") => self.print_help(),
                        Some(&"world") => self.query_world_state().await?,
                        Some(&"harmony") => self.query_harmony_levels().await?,
                        Some(&"npc") => {
                            if parts.len() >= 3 {
                                let name = parts[1].to_string();
                                let location = parts[2..].join(" ");
                                self.create_npc(name, location).await?;
                            } else {
                                println!("Usage: npc <name> <location>");
                            }
                        }
                        Some(&"quest") => {
                            if parts.len() >= 3 {
                                let quest_type = parts[1].to_string();
                                let difficulty = parts[2].parse().unwrap_or(1);
                                self.generate_quest(quest_type, difficulty).await?;
                            } else {
                                println!("Usage: quest <type> <difficulty>");
                            }
                        }
                        Some(&"event") => {
                            if parts.len() >= 2 {
                                let event_type = parts[1].to_string();
                                let params = if parts.len() > 2 {
                                    serde_json::json!({ "details": parts[2..].join(" ") })
                                } else {
                                    serde_json::json!({})
                                };
                                self.trigger_event(event_type, params).await?;
                            } else {
                                println!("Usage: event <type> [params]");
                            }
                        }
                        Some(&"raw") => {
                            if parts.len() > 1 {
                                let command = parts[1..].join(" ");
                                self.send_command(&command).await?;
                            } else {
                                println!("Usage: raw <json_command>");
                            }
                        }
                        _ => {
                            println!("Unknown command. Type 'help' for available commands.");
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C detected. Use 'exit' to quit.");
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D detected. Exiting.");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        Ok(())
    }

fn print_help(&self) {
        println!("Available commands:");
        println!("  help              - Show this help message");
        println!("  exit/quit         - Exit the CLI");
        println!("  world             - Query world state");
        println!("  harmony           - Query harmony levels");
        println!("  npc <name> <loc>  - Create an NPC");
        println!("  quest <type> <n>  - Generate a quest");
        println!("  event <type>      - Trigger an event");
        println!("  raw <json>        - Send raw JSON command");
        println!("  chat              - Enter AI chat mode");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut client = FinalverseCli::new(cli.server);
    client.connect().await?;

    match cli.command {
        Some(Commands::Interactive) | None if cli.interactive => {
            client.interactive_mode().await?;
        }
        Some(Commands::Chat) => {
            client.chat_mode().await?;
        }
        Some(Commands::Exec { command }) => {
            client.send_command(&command).await?;
        }
        Some(Commands::Shutdown) => {
            client
                .send_command(&serde_json::json!({"type": "shutdown"}).to_string())
                .await?;
        }
        _ => {
            eprintln!("Selected command not implemented in CLI yet");
        }
    }

    Ok(())
}
