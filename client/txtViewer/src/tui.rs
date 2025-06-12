use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

use crate::enhanced_client::EnhancedClient;

pub async fn run_tui(client: &mut EnhancedClient) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run_app(client, &mut terminal).await;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res
}

async fn run_app<B: ratatui::backend::Backend>(
    client: &mut EnhancedClient,
    terminal: &mut Terminal<B>,
) -> anyhow::Result<()> {
    let mut state = ListState::default();
    state.select(Some(0));
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let items = vec![
                ListItem::new("Check services"),
                ListItem::new("View world state"),
                ListItem::new("Interact with Echo"),
                ListItem::new("Quit"),
            ];
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Finalverse Client"))
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, size, &mut state);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => {
                        let i = match state.selected() { Some(i) => (i + 1) % 4, None => 0 };
                        state.select(Some(i));
                    }
                    KeyCode::Up => {
                        let i = match state.selected() { Some(0) | None => 3, Some(i) => i - 1 };
                        state.select(Some(i));
                    }
                    KeyCode::Enter => match state.selected().unwrap_or(0) {
                        0 => client.check_services().await,
                        1 => { let _ = client.view_world_state().await; },
                        2 => { let _ = client.interact_with_echo("lumi").await; },
                        3 => break,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
