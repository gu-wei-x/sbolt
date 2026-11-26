mod companies;
mod employees;
mod forms;
mod home;

pub(crate) use companies::routes as company_routes;
pub(crate) use employees::routes as employee_routes;
pub(crate) use home::routes as home_routes;

use rocket::{
    Request,
    http::Status,
    response::{self, Redirect, Responder, content::RawHtml},
};
use roster_views::models::{Page, Sidebar};

use super::AppState;
use forms::ListQuery;

pub(crate) struct AppError(Status, String);

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        rocket::Response::build_from(self.1.respond_to(request)?)
            .status(self.0)
            .ok()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        eprintln!("Database operation failed: {error}");
        if let sqlx::Error::Database(ref database) = error
            && database.is_foreign_key_violation()
        {
            return Self(
                Status::UnprocessableEntity,
                "The selected company no longer exists. Reload the form and try again.".into(),
            );
        }
        Self(
            Status::InternalServerError,
            "A database operation failed. See the server log for details.".into(),
        )
    }
}

pub(crate) enum WebResponse {
    Page(Status, String),
    Redirect(&'static str),
}

impl<'r> Responder<'r, 'static> for WebResponse {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        match self {
            Self::Page(status, html) => {
                rocket::Response::build_from((status, RawHtml(html)).respond_to(request)?)
                    .raw_header("Cache-Control", "no-store")
                    .ok()
            }
            Self::Redirect(location) => Redirect::to(location).respond_to(request),
        }
    }
}

pub(crate) type WebResult = Result<WebResponse, AppError>;

fn render(view: &str, page: Page, status: Status) -> WebResult {
    let mut context = sbolt::context!(page);
    match roster_views::render(view, None, &mut context) {
        Ok((html, _)) => Ok(WebResponse::Page(status, html)),
        Err(error) => {
            eprintln!("Template rendering failed: {error:?}");
            Err(AppError(
                Status::InternalServerError,
                "The page could not be rendered. See the server log for details.".into(),
            ))
        }
    }
}

fn redirect(location: &'static str) -> WebResult {
    Ok(WebResponse::Redirect(location))
}

fn page(state: &AppState, title: &'static str, active_nav: &'static str) -> Page {
    Page {
        title,
        sidebar: Sidebar::new(active_nav),
        framework: "Rocket",
        csrf: state.csrf.to_string(),
        ..Page::default()
    }
}

fn csrf(state: &AppState, value: &str) -> Result<(), AppError> {
    if value != state.csrf.as_ref() {
        return Err(AppError(
            Status::Forbidden,
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
    AppError(Status::NotFound, "The record no longer exists.".into())
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
