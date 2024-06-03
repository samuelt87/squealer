use sqlx::Sqlite;
use std::error::Error;

use crate::{config::Config, config::StartingDb, App};

pub async fn setup_test_database(pool: &sqlx::Pool<Sqlite>) {
    let init_query = "
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        );
        INSERT INTO users (name, email) VALUES ('Alice', 'temp@email.com')
        ";

    let _ = sqlx::query(init_query).execute(pool).await;
}

pub async fn execute_query(app: &mut App<'static>) -> Result<(), Box<dyn Error>> {
    let query = app.query_input.lines().join(" ");
    // if let Some(ref pool) = app.pool {
    match &app.pool {
        Some(pool) => {
            app.results = Some(sqlx::query(&query).fetch_all(pool).await?);
            Ok(())
        }
        //None => Err("No database connection".into()),
        None => Ok(()),
    }
}

pub async fn initial_database_conection(config: Config) -> Option<sqlx::Pool<Sqlite>> {
    match config.starting_db {
        StartingDb::File(file) => connect_to_database_file(&file).await,
        StartingDb::InMemory => match sqlx::SqlitePool::connect("sqlite::memory:").await {
            Ok(pool) => {
                setup_test_database(&pool).await;
                Some(pool)
            }
            Err(_) => None,
        },
        StartingDb::None => None,
    }
}

pub async fn connect_to_database_file(file: &str) -> Option<sqlx::Pool<Sqlite>> {
    match sqlx::SqlitePool::connect(format!("sqlite://{}", file).as_str()).await {
        Ok(pool) => Some(pool),
        Err(_) => None,
    }
}
