use ratatui::widgets::{Block, Borders};
use ratatui_explorer::FileExplorer;
use sqlx::Sqlite;
use sqlx::{sqlite::SqliteRow, Pool};
use tui_textarea::TextArea;

use crate::config::Config;

#[derive(Clone)]
pub enum UiState {
    Query,
    Explorer,
}

pub struct App<'a> {
    pub config: Config,
    pub pool: Option<Pool<Sqlite>>,
    pub results: Option<Vec<SqliteRow>>,
    pub query_input: TextArea<'a>,
    pub ui_state: UiState,
    pub table_state: ratatui::widgets::TableState,
    pub explorer: Option<FileExplorer>,
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
        let ui_state = match pool {
            Some(_) => UiState::Query,
            None => UiState::Explorer,
        };
        Self {
            config,
            pool,
            results: None,
            query_input,
            ui_state,
            table_state: ratatui::widgets::TableState::default(),
            explorer: None,
        }
    }
}
