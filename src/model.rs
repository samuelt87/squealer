use sqlx::Sqlite;
use sqlx::{sqlite::SqliteRow, Column, Pool, Row};
use std::error::Error;

use crate::config::*;
use crate::database::*;

// Modes

struct Home;
struct EditQuery;
struct BrowseSqliteDBFiles;
struct ExploreResults;
struct SaveResults;
struct ExploreConnection;
struct ConfigEditor;

pub struct App<Mode> {
    mode: Mode,
    connections: Connections,
    results: Results,
}

pub struct Connections {
    sqlite_pool: Option<Pool<Sqlite>>,
}

impl Connections {
    async fn init_connections(config: Config) -> Self {
        let sqlite_pool = initial_database_conection(config).await;
        Self { sqlite_pool }
    }

    async fn connect_to_sqlite_db(file: &str) -> Self {
        let sqlite_pool = connect_to_database_file(file).await;
        Self { sqlite_pool }
    }

    fn disconnect(self) -> Self {
        Self { sqlite_pool: None }
    }
}

pub enum Results {
    Some {
        headers: Vec<String>,
        data: Vec<Vec<String>>,
    },
    None,
}

impl Default for Results {
    fn default() -> Self {
        Self::None
    }
}

impl Results {
    fn new(rows: Vec<SqliteRow>) -> Self {
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
        Results::Some { headers, data }
    }
}

impl<T> App<T> {
    pub fn cancel(self) -> App<Home> {
        App {
            mode: Home,
            connections: self.connections,
            results: self.results,
        }
    }

    fn copy_app_with_new_mode<NewMode>(self, mode: NewMode) -> App<NewMode> {
        App {
            mode,
            connections: self.connections,
            results: self.results,
        }
    }

    pub fn add_results(self, rows: Vec<SqliteRow>) -> App<T> {
        App {
            mode: self.mode,
            connections: self.connections,
            results: Results::new(rows),
        }
    }
}

impl App<Home> {
    pub async fn new(config: Config) -> Self {
        App {
            mode: Home,
            connections: Connections::init_connections(config).await,
            results: Results::default(),
        }
    }

    pub fn edit_query(self) -> App<EditQuery> {
        self.copy_app_with_new_mode(EditQuery)
    }

    pub fn open_sqlite_db(self) -> App<BrowseSqliteDBFiles> {
        self.copy_app_with_new_mode(BrowseSqliteDBFiles)
    }

    pub fn explore_results(self) -> App<ExploreResults> {
        self.copy_app_with_new_mode(ExploreResults)
    }

    pub fn explore_connection(self) -> App<ExploreConnection> {
        self.copy_app_with_new_mode(ExploreConnection)
    }

    pub fn edit_config(self) -> App<ConfigEditor> {
        self.copy_app_with_new_mode(ConfigEditor)
    }
}

impl App<EditQuery> {}

impl App<BrowseSqliteDBFiles> {
    pub async fn open_sqlite_db(self, file: &str) -> App<Home> {
        let sqlite_pool = Connections::connect_to_sqlite_db(file).await;
        App {
            mode: Home,
            connections: sqlite_pool,
            results: self.results,
        }
    }
}

impl App<ExploreResults> {}

impl App<ExploreConnection> {}

impl App<ConfigEditor> {}
