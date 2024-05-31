mod database;
mod terminal;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use database::{execute_query, init_database};
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
            let (headers, data) = extract_row_data(&app).unwrap();
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                .split(size);

            // Render input field
            f.render_widget(app.query_input.widget(), chunks[0]);

            // Render table
            render_table(f, chunks[1], &headers, &data, &mut table_state);
        })?;

        // Handle user input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Tab => {
                    execute_query(&mut app).await?;
                }
                KeyCode::Esc => break,
                _ => {
                    app.query_input.input(key);
                }
            }
        }
    }

    restore_terminal(&mut terminal)
}

/// Execute a query from the input field and update the results.
fn render_table(
    frame: &mut Frame,
    area: Rect,
    headers: &[String],
    data: &[Vec<String>],
    state: &mut TableState,
) {
    let rows = data.iter().map(|row| {
        TableRow::new(
            row.iter()
                .map(|cell| Cell::from(cell.as_str()))
                .collect::<Vec<_>>(),
        )
    });

    let table = Table::new(rows, [15, 20, 30])
        .header(TableRow::new(
            headers
                .iter()
                .map(|h| Cell::from(h.as_str()))
                .collect::<Vec<_>>(),
        ))
        .block(Block::default().title("SQL Results").borders(Borders::ALL))
        .highlight_symbol(">>");

    frame.render_stateful_widget(table, area, state);
}

fn extract_row_data(app: &AppState) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn Error>> {
    match app.results {
        None => return Err("No results to display".into()),
        Some(ref rows) => {
            let headers = rows
                .first()
                .map(|row| {
                    row.columns()
                        .iter()
                        .map(|col| col.name().to_string())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let data = rows
                .iter()
                .map(|row| {
                    (0..row.len())
                        .map(|i| row.try_get::<&str, _>(i).unwrap_or_default().to_string())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            Ok((headers, data))
        }
    }
}
// TODO
// - Set up the chinook database for testing
