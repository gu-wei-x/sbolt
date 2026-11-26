use sqlx::{FromRow, SqlitePool};

#[derive(Debug, FromRow)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub address: String,
    pub contact_no: String,
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<Company>, sqlx::Error> {
    sqlx::query_as("SELECT id, name, address, contact_no FROM companies ORDER BY id")
        .fetch_all(pool)
        .await
}

pub async fn find(pool: &SqlitePool, id: i64) -> Result<Option<Company>, sqlx::Error> {
    sqlx::query_as("SELECT id, name, address, contact_no FROM companies WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn exists(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM companies WHERE id = ?)")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create(
    pool: &SqlitePool,
    name: &str,
    address: &str,
    contact_no: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO companies (name, address, contact_no) VALUES (?, ?, ?)")
        .bind(name)
        .bind(address)
        .bind(contact_no)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update(
    pool: &SqlitePool,
    id: i64,
    name: &str,
    address: &str,
    contact_no: &str,
) -> Result<u64, sqlx::Error> {
    let result =
        sqlx::query("UPDATE companies SET name = ?, address = ?, contact_no = ? WHERE id = ?")
            .bind(name)
            .bind(address)
            .bind(contact_no)
            .bind(id)
            .execute(pool)
            .await?;
    Ok(result.rows_affected())
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM companies WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
