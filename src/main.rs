mod app;
mod config;
mod database;
mod event_handler;
mod terminal;
mod ui;

use app::App;
use database::{connect_to_database, execute_query};
use event_handler::handle_event;
use ratatui::widgets::TableState;
use std::error::Error;
use terminal::{restore_terminal, setup_terminal};
use tokio;
use ui::render_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new("SELECT id, name, email FROM users", connect_to_database).await;

    // Fetch the results
    execute_query(&mut app).await?;

    // Create table state
    let mut table_state = TableState::default();

    let mut terminal = setup_terminal()?;
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
