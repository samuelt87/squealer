use ratatui::widgets::{Block, Borders};
use sqlx::Sqlite;
use sqlx::{sqlite::SqliteRow, Pool};
use tui_textarea::TextArea;

use crate::config::Config;

pub struct App<'a> {
    pub config: Config,
    pub pool: Option<Pool<Sqlite>>,
    pub results: Option<Vec<SqliteRow>>,
    pub query_input: TextArea<'a>,
}

impl<'a> App<'a> {
    pub async fn new<F, Fut>(start_query: &str, pool_initiliser: F) -> Self
    where
        F: FnOnce(Config) -> Fut,
        Fut: std::future::Future<Output = Option<Pool<Sqlite>>>,
    {
        let config = Config::new();
        let mut query_input = TextArea::default();
        query_input.set_block(Block::default().borders(Borders::ALL).title("Query Input"));
        query_input.insert_str(start_query);
        let pool = pool_initiliser(config.clone()).await;
        Self {
            config,
            pool,
            results: None,
            query_input,
        }
    }
}
