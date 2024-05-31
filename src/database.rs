use sqlx::{
    sqlite::{SqlitePool, SqliteRow},
    Column, Pool,
};
use sqlx::{Row, Sqlite};
use std::{error::Error, io, thread, time::Duration};

use crate::AppState;

pub async fn init_database(pool: &sqlx::Pool<Sqlite>) -> Result<(), Box<dyn Error>> {
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
