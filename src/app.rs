use ratatui::widgets::{Block, Borders};
use sqlx::Sqlite;
use sqlx::{sqlite::SqliteRow, Pool};
use tui_textarea::TextArea;

pub struct AppState<'a> {
    pub pool: Option<Pool<Sqlite>>,
    pub results: Option<Vec<SqliteRow>>,
    pub query_input: TextArea<'a>,
}

impl AppState<'static> {
    pub async fn new<F, Fut>(start_query: &str, pool_initiliser: F) -> Self
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Option<Pool<Sqlite>>>,
    {
        let mut query_input = TextArea::default();
        query_input.set_block(Block::default().borders(Borders::ALL).title("Query Input"));
        query_input.insert_str(start_query);
        Self {
            pool: pool_initiliser().await,
            results: None,
            query_input,
        }
    }
}
