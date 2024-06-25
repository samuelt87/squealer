use sqlx::Sqlite;
use sqlx::{sqlite::SqliteRow, Pool};

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
    pub connections: Connections,
    pub results: Results,
}

pub struct Connections {
    sqlite_pool: Option<Pool<Sqlite>>,
}

impl Connections {
    async fn init_connections(config: Config) -> Self {
        let sqlite_pool = initial_database_conection(config).await;
        Self { sqlite_pool }
    }
}

pub struct Results {
    results: Option<Vec<SqliteRow>>,
}

impl Results {
    fn new() -> Self {
        Self { results: None }
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
}

impl App<Home> {
    pub async fn new(config: Config) -> Self {
        App {
            mode: Home,
            connections: Connections::init_connections(config).await,
            results: Results::new(),
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
    async fn open_sqlite_db(self, file: &str) -> App<Home> {
        let sqlite_pool = connect_to_database_file(file).await;
        App {
            mode: Home,
            connections: Connections {
                sqlite_pool: sqlite_pool,
            },
            results: self.results,
        }
    }
}

impl App<ExploreResults> {}

impl App<ExploreConnection> {}

impl App<ConfigEditor> {}
