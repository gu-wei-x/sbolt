mod companies;
mod employees;
mod forms;
mod home;

pub(crate) use companies::routes as company_routes;
pub(crate) use employees::routes as employee_routes;
pub(crate) use home::home;

use std::fmt;

use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header},
};
use roster_views::models::{Page, Sidebar};

use super::AppState;
use forms::ListQuery;

#[derive(Debug)]
pub(crate) struct AppError(StatusCode, String);

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.1)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.0
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        eprintln!("Database operation failed: {error}");
        if let sqlx::Error::Database(ref database) = error
            && database.is_foreign_key_violation()
        {
            return Self(
                StatusCode::UNPROCESSABLE_ENTITY,
                "The selected company no longer exists. Reload the form and try again.".into(),
            );
        }
        Self(
            StatusCode::INTERNAL_SERVER_ERROR,
            "A database operation failed. See the server log for details.".into(),
        )
    }
}

pub(crate) type WebResult = Result<HttpResponse, AppError>;

fn render(view: &str, page: Page, status: StatusCode) -> WebResult {
    let mut context = sbolt::context!(page);
    match roster_views::render(view, None, &mut context) {
        Ok((html, _)) => Ok(HttpResponse::build(status)
            .insert_header((header::CACHE_CONTROL, "no-store"))
            .content_type("text/html; charset=utf-8")
            .body(html)),
        Err(error) => {
            eprintln!("Template rendering failed: {error:?}");
            Err(AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "The page could not be rendered. See the server log for details.".into(),
            ))
        }
    }
}

fn redirect(location: &'static str) -> WebResult {
    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, location))
        .finish())
}

fn page(state: &AppState, title: &'static str, active_nav: &'static str) -> Page {
    Page {
        title,
        sidebar: Sidebar::new(active_nav),
        framework: "Actix Web",
        csrf: state.csrf.to_string(),
        ..Page::default()
    }
}

fn csrf(state: &AppState, value: &str) -> Result<(), AppError> {
    if value != state.csrf.as_ref() {
        return Err(AppError(
            StatusCode::FORBIDDEN,
            "Invalid form token. Reload the page and try again.".into(),
        ));
    }
    Ok(())
}

fn changed(rows: u64) -> Result<(), AppError> {
    if rows == 0 {
        return Err(not_found());
    }
    Ok(())
}

fn not_found() -> AppError {
    AppError(StatusCode::NOT_FOUND, "The record no longer exists.".into())
}

fn saved(query: ListQuery) -> String {
    match query.saved.as_deref() {
        Some("created") => "Record added.".into(),
        Some("updated") => "Record updated.".into(),
        Some("deleted") => "Record deleted.".into(),
        _ => String::new(),
    }
}

fn validate_fields(fields: &[(&str, &str, usize)]) -> Result<(), String> {
    for (name, value, max) in fields {
        if value.trim().is_empty() || value.chars().count() > *max {
            return Err(format!(
                "{name} is required and must be at most {max} characters."
            ));
        }
    }
    Ok(())
}
