use rocket::{Route, State, form::Form, http::Status};
use roster_data::companies as company_data;
use roster_views::models::Company;
use sqlx::SqlitePool;

use super::forms::{CompanyForm, DeleteForm, ListQuery};
use super::{
    AppError, AppState, WebResult, changed, csrf, not_found, page, redirect, render, saved,
    validate_fields,
};

pub(crate) fn routes() -> Vec<Route> {
    rocket::routes![
        companies,
        new_company,
        edit_company,
        add_company,
        update_company,
        delete_company
    ]
}

pub(super) async fn load_companies(pool: &SqlitePool) -> Result<Vec<Company>, AppError> {
    Ok(company_data::list(pool)
        .await?
        .into_iter()
        .map(Company::from)
        .collect())
}

#[rocket::get("/companies?<query..>")]
async fn companies(state: &State<AppState>, query: ListQuery) -> WebResult {
    let mut page = page(state, "Companies", "/companies");
    page.companies = load_companies(&state.pool).await?;
    page.message = saved(query);
    render("views/companies", page, Status::Ok)
}

async fn company_form(state: &AppState, company: Company, error: String) -> WebResult {
    let mut page = page(state, "Companies", "/companies");
    page.companies = load_companies(&state.pool).await?;
    page.company = Some(company);
    page.error = error;
    let status = if page.error.is_empty() {
        Status::Ok
    } else {
        Status::UnprocessableEntity
    };
    render("views/companies", page, status)
}

#[rocket::get("/companies/new")]
async fn new_company(state: &State<AppState>) -> WebResult {
    company_form(state, Company::default(), String::new()).await
}

#[rocket::get("/companies/<id>/edit")]
async fn edit_company(state: &State<AppState>, id: i64) -> WebResult {
    let company = company_data::find(&state.pool, id)
        .await?
        .map(Company::from)
        .ok_or_else(not_found)?;
    company_form(state, company, String::new()).await
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

#[rocket::post("/companies", data = "<form>")]
async fn add_company(state: &State<AppState>, form: Form<CompanyForm>) -> WebResult {
    save_company(state, None, form.into_inner()).await
}

#[rocket::post("/companies/<id>", data = "<form>")]
async fn update_company(state: &State<AppState>, id: i64, form: Form<CompanyForm>) -> WebResult {
    save_company(state, Some(id), form.into_inner()).await
}

#[rocket::post("/companies/<id>/delete", data = "<form>")]
async fn delete_company(state: &State<AppState>, id: i64, form: Form<DeleteForm>) -> WebResult {
    csrf(state, &form.csrf)?;
    changed(company_data::delete(&state.pool, id).await?)?;
    redirect("/companies?saved=deleted")
}
