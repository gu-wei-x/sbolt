use axum::{
    Form, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use roster_data::companies as company_data;
use roster_views::models::Company;
use sqlx::SqlitePool;

use super::forms::{CompanyForm, DeleteForm, ListQuery};
use super::{
    AppError, AppState, WebResult, changed, csrf, not_found, page, render, saved, validate_fields,
};

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/companies", get(companies).post(add_company))
        .nest(
            "/companies",
            Router::new()
                .route("/new", get(new_company))
                .route("/{id}/edit", get(edit_company))
                .route("/{id}", post(update_company))
                .route("/{id}/delete", post(delete_company)),
        )
}

pub(super) async fn load_companies(pool: &SqlitePool) -> Result<Vec<Company>, AppError> {
    Ok(company_data::list(pool)
        .await?
        .into_iter()
        .map(Company::from)
        .collect())
}

async fn companies(State(state): State<AppState>, Query(query): Query<ListQuery>) -> WebResult {
    let mut page = page(&state, "Companies", "/companies");
    page.companies = load_companies(&state.pool).await?;
    page.message = saved(query);
    render("views/companies", page, StatusCode::OK)
}

async fn company_form(state: &AppState, company: Company, error: String) -> WebResult {
    let mut page = page(state, "Companies", "/companies");
    page.companies = load_companies(&state.pool).await?;
    page.company = Some(company);
    page.error = error;
    let status = if page.error.is_empty() {
        StatusCode::OK
    } else {
        StatusCode::UNPROCESSABLE_ENTITY
    };
    render("views/companies", page, status)
}

async fn new_company(State(state): State<AppState>) -> WebResult {
    company_form(&state, Company::default(), String::new()).await
}

async fn edit_company(State(state): State<AppState>, Path(id): Path<i64>) -> WebResult {
    let company = company_data::find(&state.pool, id)
        .await?
        .map(Company::from)
        .ok_or_else(not_found)?;
    company_form(&state, company, String::new()).await
}

async fn save_company(state: AppState, id: Option<i64>, form: CompanyForm) -> WebResult {
    csrf(&state, &form.csrf)?;
    let company = Company {
        id: id.unwrap_or(0),
        name: form.name,
        address: form.address,
        contact_no: form.contact_no,
    };
    if let Err(error) = validate_fields(&[
        ("Company name", &company.name, 80),
        ("Company address", &company.address, 80),
        ("Contact number", &company.contact_no, 20),
    ]) {
        return company_form(&state, company, error).await;
    }
    if let Some(id) = id {
        let rows = company_data::update(
            &state.pool,
            id,
            &company.name,
            &company.address,
            &company.contact_no,
        )
        .await?;
        changed(rows)?;
    } else {
        company_data::create(
            &state.pool,
            &company.name,
            &company.address,
            &company.contact_no,
        )
        .await?;
    }
    Ok(Redirect::to(if id.is_some() {
        "/companies?saved=updated"
    } else {
        "/companies?saved=created"
    })
    .into_response())
}

async fn add_company(State(state): State<AppState>, Form(form): Form<CompanyForm>) -> WebResult {
    save_company(state, None, form).await
}

async fn update_company(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<CompanyForm>,
) -> WebResult {
    save_company(state, Some(id), form).await
}

async fn delete_company(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<DeleteForm>,
) -> WebResult {
    csrf(&state, &form.csrf)?;
    let rows = company_data::delete(&state.pool, id).await?;
    changed(rows)?;
    Ok(Redirect::to("/companies?saved=deleted").into_response())
}
