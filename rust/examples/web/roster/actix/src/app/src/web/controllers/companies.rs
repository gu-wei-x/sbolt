use actix_web::{
    http::StatusCode,
    web::{self, Data, Form, Path, Query},
};
use roster_data::companies as company_data;
use roster_views::models::Company;
use sqlx::SqlitePool;

use super::forms::{CompanyForm, DeleteForm, ListQuery};
use super::{
    AppError, AppState, WebResult, changed, csrf, not_found, page, redirect, render, saved,
    validate_fields,
};

pub(crate) fn routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/companies")
                .route(web::get().to(companies))
                .route(web::post().to(add_company)),
        )
        .service(
            web::scope("/companies")
                .route("/new", web::get().to(new_company))
                .route("/{id}/edit", web::get().to(edit_company))
                .route("/{id}", web::post().to(update_company))
                .route("/{id}/delete", web::post().to(delete_company)),
        );
}

pub(super) async fn load_companies(pool: &SqlitePool) -> Result<Vec<Company>, AppError> {
    Ok(company_data::list(pool)
        .await?
        .into_iter()
        .map(Company::from)
        .collect())
}

async fn companies(state: Data<AppState>, query: Query<ListQuery>) -> WebResult {
    let mut page = page(&state, "Companies", "/companies");
    page.companies = load_companies(&state.pool).await?;
    page.message = saved(query.into_inner());
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

async fn new_company(state: Data<AppState>) -> WebResult {
    company_form(&state, Company::default(), String::new()).await
}

async fn edit_company(state: Data<AppState>, id: Path<i64>) -> WebResult {
    let company = company_data::find(&state.pool, id.into_inner())
        .await?
        .map(Company::from)
        .ok_or_else(not_found)?;
    company_form(&state, company, String::new()).await
}

async fn save_company(state: &AppState, id: Option<i64>, form: CompanyForm) -> WebResult {
    csrf(state, &form.csrf)?;
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
        return company_form(state, company, error).await;
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
    redirect(if id.is_some() {
        "/companies?saved=updated"
    } else {
        "/companies?saved=created"
    })
}

async fn add_company(state: Data<AppState>, form: Form<CompanyForm>) -> WebResult {
    save_company(&state, None, form.into_inner()).await
}

async fn update_company(
    state: Data<AppState>,
    id: Path<i64>,
    form: Form<CompanyForm>,
) -> WebResult {
    save_company(&state, Some(id.into_inner()), form.into_inner()).await
}

async fn delete_company(state: Data<AppState>, id: Path<i64>, form: Form<DeleteForm>) -> WebResult {
    csrf(&state, &form.csrf)?;
    changed(company_data::delete(&state.pool, id.into_inner()).await?)?;
    redirect("/companies?saved=deleted")
}
