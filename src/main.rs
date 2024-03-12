use std::str::FromStr;

use sqlx::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteRow},
    Row, SqlitePool,
};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = create_database("db01.sqlite").await?;
    println!("Created");

    update_test(&pool).await?;
    println!("Updated");

    let name = read_name(&pool).await?;
    println!("{name}");

    Ok(())
}

async fn create_database(db_url: &str) -> Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(db_url)?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;

    const SETUP: &str = r"
DROP TABLE IF EXISTS test;
CREATE TABLE IF NOT EXISTS test (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO test(id, name) VALUES(1, 'Initial value');
    ";

    sqlx::query(SETUP).execute(&pool).await?;

    Ok(pool)
}

async fn read_name(pool: &SqlitePool) -> Result<String> {
    const QUERY: &str = "SELECT name FROM test WHERE id = 1";
    sqlx::query(QUERY)
        .map(|r: SqliteRow| r.get(0))
        .fetch_one(pool)
        .await
}

async fn update_test(pool: &SqlitePool) -> Result<()> {
    const QUERY: &str = r#"
            UPDATE test SET name = 'changed value' WHERE id = 1
            RETURNING 1
            "#;
    sqlx::query(QUERY).fetch_optional(pool).await?;

    Ok(())
}
