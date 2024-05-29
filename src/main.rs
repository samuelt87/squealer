use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Cell, Row as TableRow, Table, TableState},
    Terminal,
};
use sqlx::{
    sqlite::{SqlitePool, SqliteRow},
    Column, Pool,
};
use sqlx::{Row, Sqlite};
use std::{error::Error, io, thread, time::Duration};
use tokio;

struct AppState {
    pool: Option<Pool<Sqlite>>,
    results: Option<Vec<SqliteRow>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            pool: None,
            results: None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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
    let rows = match app.pool {
        None => return Err("Failed to connect to database".into()),
        Some(ref pool) => sqlx::query(query).fetch_all(pool).await?,
    };

    // Prepare the data for the table
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

    // Create table state
    let mut table_state = TableState::default();

    let rows = data
        .iter()
        .map(|row| {
            TableRow::new(
                row.iter()
                    .map(|cell| Cell::from(cell.as_str()))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let table = Table::new(rows, [15, 20, 30])
        .header(TableRow::new(
            headers
                .iter()
                .map(|h| Cell::from(h.as_str()))
                .collect::<Vec<_>>(),
        ))
        .block(Block::default().title("SQL Results").borders(Borders::ALL))
        .highlight_symbol(">>");

    // Draw the table
    terminal.draw(|f| {
        let size = f.size();
        f.render_stateful_widget(table, size, &mut table_state);
    })?;

    // Start a thread to discard any input events. Without handling events, the
    // stdin buffer will fill up, and be read into the shell when the program exits.
    thread::spawn(|| loop {
        let _ = event::read();
    });

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn init_database(pool: &sqlx::Pool<Sqlite>) -> Result<(), Box<dyn Error>> {
    let init_query = "
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        );
        INSERT INTO users (name, email) VALUES ('Alice', 'temp@email.com')
        ";

    sqlx::query(init_query).execute(pool).await?;

    Ok(())
}

// TODO
// - Create a query output state and have it as input for drawing
// - Create a query function that updates the query output state and takes a query string
// - Create a text input box that makes a new query
// - Put results into a table instead of a paragraph
// - Set up the chinook database for testing
