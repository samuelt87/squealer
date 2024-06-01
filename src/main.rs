mod database;
mod event_handler;
mod terminal;
mod ui;

use database::{connect_to_database, execute_query, setup_test_database};
use event_handler::handle_event;
use ratatui::widgets::{Block, Borders, TableState};
use sqlx::Sqlite;
use sqlx::{sqlite::SqliteRow, Pool};
use std::error::Error;
use terminal::{restore_terminal, setup_terminal};
use tokio;
use ui::render_ui;

use tui_textarea::TextArea;

struct AppState<'a> {
    pool: Option<Pool<Sqlite>>,
    results: Option<Vec<SqliteRow>>,
    query_input: TextArea<'a>,
}

impl Default for AppState<'static> {
    fn default() -> Self {
        let mut query_input = TextArea::default();
        query_input.set_block(Block::default().borders(Borders::ALL).title("Query Input"));
        Self {
            pool: None,
            results: None,
            query_input,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;

    let mut app = AppState::default();

    connect_to_database(&mut app).await;

    match app.pool {
        None => return Err("Failed to connect to database".into()),
        Some(ref pool) => {
            setup_test_database(pool).await?;
        }
    };

    // Example dynamic query
    let query = "SELECT id, name, email FROM users"; // This could be provided at runtime
    app.query_input.insert_str(query);

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
