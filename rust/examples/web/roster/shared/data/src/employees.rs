use sqlx::{FromRow, SqlitePool};

#[derive(Debug, FromRow)]
pub struct Employee {
    pub id: i64,
    pub name: String,
    pub contact_no: String,
    pub email: String,
    pub comp_id: Option<i64>,
    pub company_name: Option<String>,
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<Employee>, sqlx::Error> {
    sqlx::query_as(
        "SELECT e.id, e.name, e.contact_no, e.email, e.comp_id, c.name AS company_name
         FROM employees e LEFT JOIN companies c ON c.id = e.comp_id ORDER BY e.id",
    )
    .fetch_all(pool)
    .await
}

pub async fn find(pool: &SqlitePool, id: i64) -> Result<Option<Employee>, sqlx::Error> {
    sqlx::query_as(
        "SELECT e.id, e.name, e.contact_no, e.email, e.comp_id, c.name AS company_name
         FROM employees e LEFT JOIN companies c ON c.id = e.comp_id WHERE e.id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &SqlitePool,
    name: &str,
    contact_no: &str,
    email: &str,
    comp_id: Option<i64>,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO employees (name, contact_no, email, comp_id) VALUES (?, ?, ?, ?)")
        .bind(name)
        .bind(contact_no)
        .bind(email)
        .bind(comp_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update(
    pool: &SqlitePool,
    id: i64,
    name: &str,
    contact_no: &str,
    email: &str,
    comp_id: Option<i64>,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE employees SET name = ?, contact_no = ?, email = ?, comp_id = ?
         WHERE id = ?",
    )
    .bind(name)
    .bind(contact_no)
    .bind(email)
    .bind(comp_id)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM employees WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
