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
        App {
            mode: EditQuery,
            connections: self.connections,
            results: self.results,
        }
    }

    pub fn open_sqlite_db(self) -> App<BrowseSqliteDBFiles> {
        App {
            mode: BrowseSqliteDBFiles,
            connections: self.connections,
            results: self.results,
        }
    }

    pub fn explore_results(self) -> App<ExploreResults> {
        App {
            mode: ExploreResults,
            connections: self.connections,
            results: self.results,
        }
    }

    pub fn explore_connection(self) -> App<ExploreConnection> {
        App {
            mode: ExploreConnection,
            connections: self.connections,
            results: self.results,
        }
    }

    pub fn edit_config(self) -> App<ConfigEditor> {
        App {
            mode: ConfigEditor,
            connections: self.connections,
            results: self.results,
        }
    }
}

impl App<EditQuery> {}

impl App<BrowseSqliteDBFiles> {}

impl App<ExploreResults> {}

impl App<ExploreConnection> {}

impl App<ConfigEditor> {}
