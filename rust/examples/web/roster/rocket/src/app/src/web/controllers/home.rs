use rocket::{Route, State, http::Status};

use super::{AppState, WebResult, page, render};

pub(crate) fn routes() -> Vec<Route> {
    rocket::routes![home]
}

#[rocket::get("/")]
async fn home(state: &State<AppState>) -> WebResult {
    let mut page = page(state, "Overview", "/");
    (page.company_count, page.employee_count) = roster_data::dashboard_counts(&state.pool).await?;
    render("views/home", page, Status::Ok)
}
