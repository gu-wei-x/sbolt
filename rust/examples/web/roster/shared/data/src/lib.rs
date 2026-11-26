use std::str::FromStr;

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

pub mod companies;
pub mod employees;

pub async fn create_pool(url: &str) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let options = SqliteConnectOptions::from_str(url)?
        .create_if_missing(true)
        .foreign_keys(true)
        .busy_timeout(std::time::Duration::from_secs(5));
    let pool = SqlitePoolOptions::new()
        .max_connections(if url == "sqlite::memory:" { 1 } else { 5 })
        .connect_with(options)
        .await?;
    sqlx::migrate!("./sql_scripts").run(&pool).await?;
    Ok(pool)
}

pub async fn dashboard_counts(pool: &SqlitePool) -> Result<(i64, i64), sqlx::Error> {
    sqlx::query_as("SELECT (SELECT COUNT(*) FROM companies), (SELECT COUNT(*) FROM employees)")
        .fetch_one(pool)
        .await
}
