use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use sqlx::SqlitePool;
use tower::ServiceExt;

async fn setup() -> (Router, SqlitePool, String) {
    let pool = roster_axum::create_pool("sqlite::memory:").await.unwrap();
    let assets = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("..")
        .join("shared")
        .join("assets");
    let app = roster_axum::create_router(pool.clone(), assets);
    let (_, html) = request(&app, "GET", "/companies/new", "").await;
    let token = html
        .split("name=\"csrf\" value=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap()
        .to_string();
    (app, pool, token)
}

async fn request(app: &Router, method: &str, path: &str, body: &str) -> (StatusCode, String) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8(bytes.to_vec()).unwrap())
}

#[tokio::test]
async fn assets_are_read_from_disk_and_can_change_without_rebuilding() {
    let assets = std::env::temp_dir().join(format!("roster-assets-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&assets).unwrap();
    let styles = assets.join("styles");
    std::fs::create_dir(&styles).unwrap();
    let stylesheet = styles.join("style.css");
    std::fs::write(&stylesheet, "body { color: red; }").unwrap();
    let pool = roster_axum::create_pool("sqlite::memory:").await.unwrap();
    let app = roster_axum::create_router(pool, assets.clone());
    let (status, content) = request(&app, "GET", "/assets/styles/style.css", "").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(content, "body { color: red; }");
    std::fs::write(&stylesheet, "body { color: blue; }").unwrap();
    let (status, content) = request(&app, "GET", "/assets/styles/style.css", "").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(content, "body { color: blue; }");
    assert_eq!(
        request(&app, "GET", "/assets/scripts/missing.js", "")
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    std::fs::remove_file(stylesheet).unwrap();
    std::fs::remove_dir(styles).unwrap();
    std::fs::remove_dir(assets).unwrap();
}

#[tokio::test]
async fn overview_shows_live_totals_and_quick_actions() {
    let (app, pool, _) = setup().await;
    for (companies, employees) in [(2, 2), (2, 1), (0, 0)] {
        let (status, html) = request(&app, "GET", "/", "").await;
        assert_eq!(status, StatusCode::OK);
        assert!(html.contains(&format!("id=\"company-count\">{companies}</p>")));
        assert!(html.contains(&format!("id=\"employee-count\">{employees}</p>")));
        assert!(html.contains("href=\"/companies/new\">+ Add company</a>"));
        assert!(html.contains("href=\"/employees/new\">+ Add employee</a>"));
        if employees == 2 {
            sqlx::query("DELETE FROM employees WHERE id = 1")
                .execute(&pool)
                .await
                .unwrap();
        } else if employees == 1 {
            sqlx::query("DELETE FROM companies")
                .execute(&pool)
                .await
                .unwrap();
        }
    }
}

#[tokio::test]
async fn navigation_marks_the_current_section_including_forms() {
    let (app, _, _) = setup().await;
    for (path, active) in [
        ("/", "/"),
        ("/companies", "/companies"),
        ("/companies/new", "/companies"),
        ("/companies/1/edit", "/companies"),
        ("/employees", "/employees"),
        ("/employees/new", "/employees"),
        ("/employees/1/edit", "/employees"),
    ] {
        let (status, html) = request(&app, "GET", path, "").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(html.matches("aria-current=\"page\"").count(), 1);
        assert!(html.contains(&format!("href=\"{active}\" aria-current=\"page\"")));
    }
}

#[tokio::test]
async fn company_and_employee_crud_preserves_assignment_and_cascades() {
    let (app, pool, token) = setup().await;
    let (status, home) = request(&app, "GET", "/", "").await;
    assert_eq!(status, StatusCode::OK);
    assert!(home.contains("<h1>Overview</h1>"));
    let (status, _) = request(
        &app,
        "POST",
        "/companies",
        &format!("csrf={token}&name=Test+Company&address=Test+Street&contact_no=555-0300"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let company: i64 = sqlx::query_scalar("SELECT id FROM companies WHERE name = 'Test Company'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let (status, edit) = request(&app, "GET", &format!("/companies/{company}/edit"), "").await;
    assert_eq!(status, StatusCode::OK);
    assert!(edit.contains("name=\"name\" value=\"Test Company\""));
    let (status, _) = request(
        &app,
        "POST",
        &format!("/companies/{company}"),
        &format!("csrf={token}&name=Renamed&address=Updated+Street&contact_no=555-0301"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, list) = request(&app, "GET", "/companies", "").await;
    assert!(list.contains("Renamed") && list.contains("Updated Street"));

    let (status, _) = request(
        &app,
        "POST",
        "/employees",
        &format!("csrf={token}&name=Test+Employee&contact_no=555-0400&email=test%40example.test&comp_id={company}"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let employee: i64 = sqlx::query_scalar("SELECT id FROM employees WHERE name = 'Test Employee'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let (status, edit) = request(&app, "GET", &format!("/employees/{employee}/edit"), "").await;
    assert_eq!(status, StatusCode::OK);
    assert!(edit.contains("name=\"name\" value=\"Test Employee\""));
    assert!(edit.contains(&format!("value=\"{company}\" selected>Renamed</option>")));
    assert!(edit.contains("<td>Test Employee</td>"));
    assert!(edit.contains("<td>Renamed</td>"));
    assert!(edit.contains("value=\"test@example.test\""));
    let (status, _) = request(
        &app,
        "POST",
        &format!("/employees/{employee}"),
        &format!("csrf={token}&name=Updated+Employee&contact_no=555-0401&email=updated%40example.test&comp_id=1"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, list) = request(&app, "GET", "/employees", "").await;
    assert!(list.contains("Updated Employee") && list.contains("updated@example.test"));
    let assignment: i64 = sqlx::query_scalar("SELECT comp_id FROM employees WHERE id = ?")
        .bind(employee)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(assignment, 1);
    let (status, _) = request(
        &app,
        "POST",
        &format!("/employees/{employee}/delete"),
        &format!("csrf={token}"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        request(&app, "GET", &format!("/employees/{employee}/edit"), "")
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    // A company deletion must remove its employees, but not unrelated employees.
    let (status, _) = request(
        &app,
        "POST",
        "/companies/1/delete",
        &format!("csrf={token}"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let remaining: Vec<Option<i64>> = sqlx::query_scalar("SELECT comp_id FROM employees")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(remaining, vec![Some(2)]);
    let (status, _) = request(
        &app,
        "POST",
        &format!("/companies/{company}/delete"),
        &format!("csrf={token}"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        request(&app, "GET", &format!("/companies/{company}/edit"), "")
            .await
            .0,
        StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn invalid_input_keeps_form_values_and_does_not_mutate_data() {
    let (app, pool, token) = setup().await;
    for (name, contact) in [
        ("".to_string(), "555".to_string()),
        ("x".repeat(81), "555".into()),
        ("Name".into(), "x".repeat(21)),
    ] {
        let (status, html) = request(
            &app,
            "POST",
            "/companies",
            &format!("csrf={token}&name={name}&address=Preserved&contact_no={contact}"),
        )
        .await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert!(html.contains("value=\"Preserved\"") && html.contains("role=\"alert\""));
    }
    for selection in ["", "0", "9999", "invalid"] {
        let (status, _) = request(
            &app,
            "POST",
            "/employees",
            &format!("csrf={token}&name=Retained&contact_no=555&email=test%40example.test&comp_id={selection}"),
        ).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }
    for (table, expected) in [("companies", 2_i64), ("employees", 2)] {
        let count: i64 = sqlx::query_scalar(&format!("SELECT COUNT(*) FROM {table}"))
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, expected);
    }
    assert_eq!(
        request(
            &app,
            "POST",
            "/companies/999",
            &format!("csrf={token}&name=Name&address=Address&contact_no=555")
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        request(
            &app,
            "POST",
            "/employees/999/delete",
            &format!("csrf={token}")
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        request(&app, "POST", "/companies/1/delete", "csrf=wrong")
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(&app, "GET", "/companies/1/delete", "").await.0,
        StatusCode::METHOD_NOT_ALLOWED
    );
}

#[tokio::test]
async fn views_escape_stored_values_and_show_empty_states() {
    let (app, pool, token) = setup().await;
    let (status, _) = request(
        &app, "POST", "/companies",
        &format!("csrf={token}&name=%3Cscript%3Ealert%281%29%3C%2Fscript%3E&address=%22+onfocus%3D%22bad&contact_no=555"),
    ).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, html) = request(&app, "GET", "/companies", "").await;
    assert!(!html.contains("<script>alert(1)</script>"));
    assert!(html.contains("&lt;script&gt;"));
    let (_, html) = request(&app, "GET", "/companies/3/edit", "").await;
    assert!(!html.contains("value=\"\" onfocus=\"bad"));
    sqlx::query("DELETE FROM companies")
        .execute(&pool)
        .await
        .unwrap();
    for path in ["/companies", "/employees"] {
        let (status, html) = request(&app, "GET", path, "").await;
        assert_eq!(status, StatusCode::OK);
        assert!(html.contains("No data for this request!"));
    }
    let (_, html) = request(&app, "GET", "/employees/new", "").await;
    assert!(html.contains("Create a company before adding an employee."));
}

#[tokio::test]
async fn database_is_persistent_and_seeded_only_once() {
    let path = std::env::temp_dir().join(format!("roster-{}.db", uuid::Uuid::new_v4()));
    let url = format!("sqlite://{}", path.display());
    let pool = roster_axum::create_pool(&url).await.unwrap();
    sqlx::query("UPDATE companies SET name = 'Persistent' WHERE id = 1")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM employees WHERE id = 2")
        .execute(&pool)
        .await
        .unwrap();
    pool.close().await;
    let pool = roster_axum::create_pool(&url).await.unwrap();
    let name: String = sqlx::query_scalar("SELECT name FROM companies WHERE id = 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(name, "Persistent");
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM employees")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
    pool.close().await;
    std::fs::remove_file(path).unwrap();
}
