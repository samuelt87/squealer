use sqlx::Sqlite;
use std::error::Error;

use crate::{config::Config, App};

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
    if let Some(ref pool) = app.pool {
        app.results = Some(sqlx::query(&query).fetch_all(pool).await?);
    }
    Ok(())
}

pub async fn connect_to_database(config: Config) -> Option<sqlx::Pool<Sqlite>> {
    match config.db_file {
        Some(file) => {
            match sqlx::SqlitePool::connect(format!("sqlite://{}", file).as_str()).await {
                Ok(pool) => Some(pool),
                Err(_) => None,
            }
        }
        None => match sqlx::SqlitePool::connect("sqlite::memory:").await {
            Ok(pool) => {
                setup_test_database(&pool).await;
                Some(pool)
            }
            Err(_) => None,
        },
    }
}
