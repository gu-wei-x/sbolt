mod companies;
mod employees;
mod forms;
mod home;

pub(crate) use companies::routes as company_routes;
pub(crate) use employees::routes as employee_routes;
pub(crate) use home::home;

use super::AppState;
use axum::{
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
};
use forms::ListQuery;
use roster_views::models::{Page, Sidebar};

pub(crate) struct AppError(StatusCode, String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
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

pub(crate) type WebResult = Result<Response, AppError>;

fn render(view: &str, page: Page, status: StatusCode) -> WebResult {
    let mut context = sbolt::context!(page);
    match roster_views::render(view, None, &mut context) {
        Ok((html, _)) => {
            Ok((status, [(header::CACHE_CONTROL, "no-store")], Html(html)).into_response())
        }
        Err(error) => {
            eprintln!("Template rendering failed: {error:?}");
            Err(AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "The page could not be rendered. See the server log for details.".into(),
            ))
        }
    }
}

fn page(state: &AppState, title: &'static str, active_nav: &'static str) -> Page {
    Page {
        title,
        sidebar: Sidebar::new(active_nav),
        framework: "Axum",
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
