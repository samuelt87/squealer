mod app;
mod database;
mod event_handler;
mod terminal;
mod ui;

use app::AppState;
use database::{connect_to_database, execute_query, setup_test_database};
use event_handler::handle_event;
use ratatui::widgets::TableState;
use std::error::Error;
use terminal::{restore_terminal, setup_terminal};
use tokio;
use ui::render_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;

    let mut app = AppState::new("SELECT id, name, email FROM users", connect_to_database).await;

    match app.pool {
        None => return Err("Failed to connect to database".into()),
        Some(ref pool) => {
            setup_test_database(pool).await?;
        }
    };

    // Fetch the results
    execute_query(&mut app).await?;

    // Create table state
    let mut table_state = TableState::default();

    loop {
        // Render the input field and table
        terminal.draw(|f| {
            render_ui(f, &app, &mut table_state).unwrap();
        })?;

        // Handle user input
        if handle_event(&mut app).await? {
            break;
        }
    }

    restore_terminal(&mut terminal)
}
