use actix_web::{
    http::StatusCode,
    web::{self, Data, Form, Path, Query},
};
use roster_data::{companies as company_data, employees as employee_data};
use roster_views::models::Employee;
use sqlx::SqlitePool;

use super::forms::{DeleteForm, EmployeeForm, ListQuery};
use super::{
    AppError, AppState, WebResult, changed, companies::load_companies, csrf, not_found, page,
    redirect, render, saved, validate_fields,
};

pub(crate) fn routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/employees")
                .route(web::get().to(employees))
                .route(web::post().to(add_employee)),
        )
        .service(
            web::scope("/employees")
                .route("/new", web::get().to(new_employee))
                .route("/{id}/edit", web::get().to(edit_employee))
                .route("/{id}", web::post().to(update_employee))
                .route("/{id}/delete", web::post().to(delete_employee)),
        );
}

async fn load_employees(pool: &SqlitePool) -> Result<Vec<Employee>, AppError> {
    Ok(employee_data::list(pool)
        .await?
        .into_iter()
        .map(Employee::from)
        .collect())
}

async fn employees(state: Data<AppState>, query: Query<ListQuery>) -> WebResult {
    let mut page = page(&state, "Employees", "/employees");
    page.employees = load_employees(&state.pool).await?;
    page.message = saved(query.into_inner());
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

async fn new_employee(state: Data<AppState>) -> WebResult {
    employee_form(&state, Employee::default(), String::new()).await
}

async fn edit_employee(state: Data<AppState>, id: Path<i64>) -> WebResult {
    let employee = employee_data::find(&state.pool, id.into_inner())
        .await?
        .map(Employee::from)
        .ok_or_else(not_found)?;
    employee_form(&state, employee, String::new()).await
}

async fn save_employee(state: &AppState, id: Option<i64>, form: EmployeeForm) -> WebResult {
    csrf(state, &form.csrf)?;
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
        return employee_form(state, employee, error).await;
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
    redirect(if id.is_some() {
        "/employees?saved=updated"
    } else {
        "/employees?saved=created"
    })
}

async fn add_employee(state: Data<AppState>, form: Form<EmployeeForm>) -> WebResult {
    save_employee(&state, None, form.into_inner()).await
}

async fn update_employee(
    state: Data<AppState>,
    id: Path<i64>,
    form: Form<EmployeeForm>,
) -> WebResult {
    save_employee(&state, Some(id.into_inner()), form.into_inner()).await
}

async fn delete_employee(
    state: Data<AppState>,
    id: Path<i64>,
    form: Form<DeleteForm>,
) -> WebResult {
    csrf(&state, &form.csrf)?;
    changed(employee_data::delete(&state.pool, id.into_inner()).await?)?;
    redirect("/employees?saved=deleted")
}
