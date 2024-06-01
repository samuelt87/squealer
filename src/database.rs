use sqlx::Sqlite;
use std::error::Error;

use crate::AppState;

pub async fn setup_test_database(pool: &sqlx::Pool<Sqlite>) -> Result<(), Box<dyn Error>> {
    let init_query = "
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        );
        INSERT INTO users (name, email) VALUES ('Alice', 'temp@email.com')
        ";

    sqlx::query(init_query).execute(pool).await?;

    Ok(())
}

pub async fn execute_query(app: &mut AppState<'static>) -> Result<(), Box<dyn Error>> {
    let query = app.query_input.lines().join(" ");
    if let Some(ref pool) = app.pool {
        app.results = Some(sqlx::query(&query).fetch_all(pool).await?);
    }
    Ok(())
}

pub async fn connect_to_database(app: &mut AppState<'static>) {
    let database_url = "sqlite::memory:"; // Use your database URL
    app.pool = match sqlx::SqlitePool::connect(database_url).await {
        Ok(pool) => Some(pool),
        Err(_) => None,
    };
}
