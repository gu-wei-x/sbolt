use actix_files::Files;
use actix_web::{
    App,
    http::{StatusCode, header},
    test,
    web::Data,
};
use roster_actix::{AppState, configure, create_pool};

macro_rules! request {
    ($app:expr, $method:ident, $path:expr, $body:expr) => {{
        let response = test::call_service(
            $app,
            test::TestRequest::$method()
                .uri($path)
                .insert_header((header::CONTENT_TYPE, "application/x-www-form-urlencoded"))
                .set_payload(($body).to_string())
                .to_request(),
        )
        .await;
        let status = response.status();
        let body = test::read_body(response).await;
        (status, String::from_utf8(body.to_vec()).unwrap())
    }};
}

#[actix_web::test]
async fn routes_render_shared_views_and_preserve_crud_and_cascades() {
    let pool = create_pool("sqlite::memory:").await.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::new(pool.clone())))
            .configure(configure)
            .service(Files::new(
                "/assets",
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../shared/assets"),
            )),
    )
    .await;
    for (path, active) in [
        ("/", "/"),
        ("/companies", "/companies"),
        ("/companies/new", "/companies"),
        ("/companies/1/edit", "/companies"),
        ("/employees", "/employees"),
        ("/employees/new", "/employees"),
        ("/employees/1/edit", "/employees"),
    ] {
        let (status, html) = request!(&app, get, path, "");
        assert_eq!(status, StatusCode::OK, "{path}");
        assert!(html.contains(&format!("href=\"{active}\" aria-current=\"page\"")));
    }
    let response = test::call_service(&app, test::TestRequest::get().uri("/").to_request()).await;
    assert_eq!(
        response.headers().get(header::CACHE_CONTROL).unwrap(),
        "no-store"
    );
    let (status, css) = request!(&app, get, "/assets/styles/style.css", "");
    assert_eq!(status, StatusCode::OK);
    assert!(css.contains(":root"));
    let (_, html) = request!(&app, get, "/companies/new", "");
    let token = html
        .split("name=\"csrf\" value=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    let (status, _) = request!(
        &app,
        post,
        "/companies",
        format!("csrf={token}&name=Test&address=Street&contact_no=555")
    );
    assert_eq!(status, StatusCode::SEE_OTHER);
    let company = roster_data::companies::list(&pool)
        .await
        .unwrap()
        .pop()
        .unwrap()
        .id;
    let (status, _) = request!(
        &app,
        post,
        &format!("/companies/{company}"),
        format!("csrf={token}&name=Renamed&address=Updated&contact_no=556")
    );
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (status, html) = request!(&app, get, &format!("/companies/{company}/edit"), "");
    assert_eq!(status, StatusCode::OK);
    assert!(html.contains("value=\"Renamed\""));
    let (status, _) = request!(
        &app,
        post,
        "/employees",
        format!(
            "csrf={token}&name=Person&contact_no=555&email=test%40example.test&comp_id={company}"
        )
    );
    assert_eq!(status, StatusCode::SEE_OTHER);
    let employee = roster_data::employees::list(&pool)
        .await
        .unwrap()
        .pop()
        .unwrap()
        .id;
    let (status, _) = request!(
        &app,
        post,
        &format!("/employees/{employee}"),
        format!("csrf={token}&name=Updated&contact_no=556&email=new%40example.test&comp_id=1")
    );
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, html) = request!(&app, get, &format!("/employees/{employee}/edit"), "");
    assert!(html.contains("value=\"new@example.test\""));
    assert_eq!(
        roster_data::employees::find(&pool, employee)
            .await
            .unwrap()
            .unwrap()
            .comp_id,
        Some(1)
    );
    let (status, _) = request!(
        &app,
        post,
        &format!("/employees/{employee}/delete"),
        format!("csrf={token}")
    );
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        request!(&app, get, &format!("/employees/{employee}/edit"), "").0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        request!(&app, post, "/companies/1/delete", format!("csrf={token}")).0,
        StatusCode::SEE_OTHER
    );
    assert_eq!(roster_data::employees::list(&pool).await.unwrap().len(), 1);
    assert_eq!(
        request!(
            &app,
            post,
            &format!("/companies/{company}/delete"),
            format!("csrf={token}")
        )
        .0,
        StatusCode::SEE_OTHER
    );
    assert_eq!(
        request!(&app, get, &format!("/companies/{company}/edit"), "").0,
        StatusCode::NOT_FOUND
    );
}

#[actix_web::test]
async fn forms_validate_tokens_input_and_html_escaping() {
    let pool = create_pool("sqlite::memory:").await.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::new(pool.clone())))
            .configure(configure),
    )
    .await;
    let (_, html) = request!(&app, get, "/companies/new", "");
    let token = html
        .split("name=\"csrf\" value=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    assert_eq!(
        request!(&app, post, "/companies/1/delete", "csrf=wrong").0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request!(&app, get, "/companies/1/delete", "").0,
        StatusCode::METHOD_NOT_ALLOWED
    );
    let (status, html) = request!(
        &app,
        post,
        "/companies",
        format!("csrf={token}&name=&address=Preserved&contact_no=555")
    );
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(html.contains("value=\"Preserved\""));
    for selection in ["", "invalid", "9999"] {
        assert_eq!(request!(&app, post, "/employees",
            format!("csrf={token}&name=Retained&contact_no=555&email=test%40example.test&comp_id={selection}")).0,
            StatusCode::UNPROCESSABLE_ENTITY);
    }
    assert_eq!(roster_data::dashboard_counts(&pool).await.unwrap(), (2, 2));
    assert_eq!(
        request!(
            &app,
            post,
            "/companies/999",
            format!("csrf={token}&name=Missing&address=Address&contact_no=555")
        )
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        request!(
            &app,
            post,
            "/companies",
            format!("csrf={token}&name=%3Cscript%3E&address=Street&contact_no=555")
        )
        .0,
        StatusCode::SEE_OTHER
    );
    let (_, html) = request!(&app, get, "/companies?saved=created", "");
    assert!(html.contains("&lt;script&gt;"));
    assert!(!html.contains("<td><script>"));
    assert!(html.contains("Record added."));
}
