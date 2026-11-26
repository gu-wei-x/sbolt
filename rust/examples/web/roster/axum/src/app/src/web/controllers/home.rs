use axum::{extract::State, http::StatusCode};

use super::{AppState, WebResult, page, render};

pub(crate) async fn home(State(state): State<AppState>) -> WebResult {
    let mut page = page(&state, "Overview", "/");
    (page.company_count, page.employee_count) = roster_data::dashboard_counts(&state.pool).await?;
    render("views/home", page, StatusCode::OK)
}
