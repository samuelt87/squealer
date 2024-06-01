use sqlx::Sqlite;
use sqlx::{sqlite::SqliteRow, Pool};
use ratatui::widgets::{Block, Borders};
use tui_textarea::TextArea;

pub struct AppState<'a> {
    pub pool: Option<Pool<Sqlite>>,
    pub results: Option<Vec<SqliteRow>>,
    pub query_input: TextArea<'a>,
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
