use sqlx::Sqlite;
use sqlx::{sqlite::SqliteRow, Pool};

use crate::config::*;
use crate::database::*;

// Modes

struct home;
struct edit_query;
struct sqlite_db_browser;
struct explore_results;
struct save_results;
struct explore_connection;
struct config_editor;

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

impl App<home> {
    async fn new(config: Config) -> Self {
        App {
            mode: home,
            connections: Connections::init_connections(config).await,
            results: Results::new(),
        }
    }

    fn edit_query(self) -> App<edit_query> {
        App {
            mode: edit_query,
            connections: self.connections,
            results: self.results,
        }
    }

    fn open_sqlite_db(self) -> App<sqlite_db_browser> {
        App {
            mode: sqlite_db_browser,
            connections: self.connections,
            results: self.results,
        }
    }

    fn explore_results(self) -> App<explore_results> {
        App {
            mode: explore_results,
            connections: self.connections,
            results: self.results,
        }
    }

    fn explore_connection(self) -> App<explore_connection> {
        App {
            mode: explore_connection,
            connections: self.connections,
            results: self.results,
        }
    }

    fn edit_config(self) -> App<config_editor> {
        App {
            mode: config_editor,
            connections: self.connections,
            results: self.results,
        }
    }
}

impl App<edit_query> {
    fn cancel(self) -> App<home> {
        App {
            mode: home,
            connections: self.connections,
            results: self.results,
        }
    }
}

impl App<sqlite_db_browser> {
    fn cancel(self) -> App<home> {
        App {
            mode: home,
            connections: self.connections,
            results: self.results,
        }
    }
}

impl App<explore_results> {
    fn cancel(self) -> App<home> {
        App {
            mode: home,
            connections: self.connections,
            results: self.results,
        }
    }
}

impl App<explore_connection> {
    fn cancel(self) -> App<home> {
        App {
            mode: home,
            connections: self.connections,
            results: self.results,
        }
    }
}

impl App<config_editor> {
    fn cancel(self) -> App<home> {
        App {
            mode: home,
            connections: self.connections,
            results: self.results,
        }
    }
}
