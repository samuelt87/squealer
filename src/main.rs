mod app;
mod config;
mod database;
mod event_handler;
mod model;
mod terminal;
mod ui;

use app::App;
use database::initial_database_conection;
use event_handler::handle_event;
use std::error::Error;
use terminal::{restore_terminal, setup_terminal};
use tokio;
use ui::render_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new(
        "SELECT id, name, email FROM users",
        initial_database_conection,
    )
    .await;

    let mut terminal = setup_terminal()?;
    loop {
        // Render the input field and table
        terminal.draw(|f| {
            render_ui(f, &mut app).unwrap();
        })?;

        // Handle user input
        if handle_event(&mut app).await? {
            break;
        }
    }

    restore_terminal(&mut terminal)
}
