use axum::{
    Form, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use roster_data::{companies as company_data, employees as employee_data};
use roster_views::models::Employee;
use sqlx::SqlitePool;

use super::forms::{DeleteForm, EmployeeForm, ListQuery};
use super::{
    AppError, AppState, WebResult, changed, companies::load_companies, csrf, not_found, page,
    render, saved, validate_fields,
};

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/employees", get(employees).post(add_employee))
        .nest(
            "/employees",
            Router::new()
                .route("/new", get(new_employee))
                .route("/{id}/edit", get(edit_employee))
                .route("/{id}", post(update_employee))
                .route("/{id}/delete", post(delete_employee)),
        )
}

async fn load_employees(pool: &SqlitePool) -> Result<Vec<Employee>, AppError> {
    Ok(employee_data::list(pool)
        .await?
        .into_iter()
        .map(Employee::from)
        .collect())
}

async fn employees(State(state): State<AppState>, Query(query): Query<ListQuery>) -> WebResult {
    let mut page = page(&state, "Employees", "/employees");
    page.employees = load_employees(&state.pool).await?;
    page.message = saved(query);
    render("views/employees", page, StatusCode::OK)
}

async fn employee_form(state: &AppState, employee: Employee, error: String) -> WebResult {
    let mut page = page(state, "Employees", "/employees");
    page.companies = load_companies(&state.pool).await?;
    page.employees = load_employees(&state.pool).await?;
    page.employee = Some(employee);
    page.error = error;
    let status = if page.error.is_empty() {
        StatusCode::OK
    } else {
        StatusCode::UNPROCESSABLE_ENTITY
    };
    render("views/employees", page, status)
}

async fn new_employee(State(state): State<AppState>) -> WebResult {
    employee_form(&state, Employee::default(), String::new()).await
}

async fn edit_employee(State(state): State<AppState>, Path(id): Path<i64>) -> WebResult {
    let employee = employee_data::find(&state.pool, id)
        .await?
        .map(Employee::from)
        .ok_or_else(not_found)?;
    employee_form(&state, employee, String::new()).await
}

async fn save_employee(state: AppState, id: Option<i64>, form: EmployeeForm) -> WebResult {
    csrf(&state, &form.csrf)?;
    let comp_id = if form.comp_id.is_empty() {
        None
    } else {
        Some(form.comp_id.parse::<i64>().map_err(|_| {
            AppError(
                StatusCode::UNPROCESSABLE_ENTITY,
                "Invalid company selection.".into(),
            )
        })?)
    };
    let employee = Employee {
        id: id.unwrap_or(0),
        name: form.name,
        contact_no: form.contact_no,
        email: form.email,
        comp_id,
        company_name: None,
    };
    let mut validation = validate_fields(&[
        ("Employee name", &employee.name, 80),
        ("Contact number", &employee.contact_no, 20),
        ("Email", &employee.email, 80),
    ]);
    if validation.is_ok() {
        let exists = match comp_id {
            Some(id) => company_data::exists(&state.pool, id).await?,
            None => false,
        };
        if !exists {
            validation = Err("Select an existing company.".into());
        }
    }
    if let Err(error) = validation {
        return employee_form(&state, employee, error).await;
    }
    if let Some(id) = id {
        let rows = employee_data::update(
            &state.pool,
            id,
            &employee.name,
            &employee.contact_no,
            &employee.email,
            comp_id,
        )
        .await?;
        changed(rows)?;
    } else {
        employee_data::create(
            &state.pool,
            &employee.name,
            &employee.contact_no,
            &employee.email,
            comp_id,
        )
        .await?;
    }
    Ok(Redirect::to(if id.is_some() {
        "/employees?saved=updated"
    } else {
        "/employees?saved=created"
    })
    .into_response())
}

async fn add_employee(State(state): State<AppState>, Form(form): Form<EmployeeForm>) -> WebResult {
    save_employee(state, None, form).await
}

async fn update_employee(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<EmployeeForm>,
) -> WebResult {
    save_employee(state, Some(id), form).await
}

async fn delete_employee(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<DeleteForm>,
) -> WebResult {
    csrf(&state, &form.csrf)?;
    let rows = employee_data::delete(&state.pool, id).await?;
    changed(rows)?;
    Ok(Redirect::to("/employees?saved=deleted").into_response())
}
