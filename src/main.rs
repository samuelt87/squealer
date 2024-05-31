mod database;
mod event_handler;
mod terminal;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use database::{execute_query, init_database};
use event_handler::handle_event;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    prelude::*,
    widgets::{Block, Borders, Cell, Row as TableRow, Table, TableState},
    Frame, Terminal,
};
use sqlx::{
    sqlite::{SqlitePool, SqliteRow},
    Column, Pool,
};
use sqlx::{Row, Sqlite};
use std::{error::Error, io, thread, time::Duration};
use terminal::{restore_terminal, setup_terminal};
use tokio;
use ui::render_ui;

use tui_textarea::{Input, TextArea};

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

    let database_url = "sqlite::memory:"; // Use your database URL
    app.pool = Some(SqlitePool::connect(database_url).await?);

    match app.pool {
        None => return Err("Failed to connect to database".into()),
        Some(ref pool) => {
            init_database(pool).await?;
        }
    };

    // Example dynamic query
    let query = "SELECT id, name, email FROM users"; // This could be provided at runtime

    // Fetch the results
    app.results = match app.pool {
        None => None,
        Some(ref pool) => Some(sqlx::query(query).fetch_all(pool).await?),
    };

    // Create table state
    let mut table_state = TableState::default();

    loop {
        // Render the input field and table
        terminal.draw(|f| {
            render_ui(f, &app, &mut table_state);
        });

        // Handle user input
        if handle_event(&mut app).await? {
            break;
        }
    }

    restore_terminal(&mut terminal)
}

// TODO
// - Set up the chinook database for testing
