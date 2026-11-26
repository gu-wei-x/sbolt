use actix_web::{http::StatusCode, web::Data};

use super::{AppState, WebResult, page, render};

pub(crate) async fn home(state: Data<AppState>) -> WebResult {
    let mut page = page(&state, "Overview", "/");
    (page.company_count, page.employee_count) = roster_data::dashboard_counts(&state.pool).await?;
    render("views/home", page, StatusCode::OK)
}
