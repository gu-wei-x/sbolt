use rocket::{
    http::{ContentType, Method, Status},
    local::asynchronous::Client,
};
use roster_rocket::{create_pool, create_router};

async fn request(client: &Client, method: Method, path: &str, body: &str) -> (Status, String) {
    let response = client
        .req(method, path)
        .header(ContentType::Form)
        .body(body)
        .dispatch()
        .await;
    let status = response.status();
    (status, response.into_string().await.unwrap_or_default())
}

fn assets() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../shared/assets")
}

#[rocket::async_test]
async fn routes_render_shared_views_and_preserve_crud_and_cascades() {
    let pool = create_pool("sqlite::memory:").await.unwrap();
    let client = Client::tracked(create_router(pool.clone(), assets()))
        .await
        .unwrap();
    for (path, active) in [
        ("/", "/"),
        ("/companies", "/companies"),
        ("/companies/new", "/companies"),
        ("/companies/1/edit", "/companies"),
        ("/employees", "/employees"),
        ("/employees/new", "/employees"),
        ("/employees/1/edit", "/employees"),
    ] {
        let (status, html) = request(&client, Method::Get, path, "").await;
        assert_eq!(status, Status::Ok, "{path}");
        assert!(html.contains(&format!("href=\"{active}\" aria-current=\"page\"")));
    }
    let response = client.get("/").dispatch().await;
    assert_eq!(
        response.headers().get_one("Cache-Control"),
        Some("no-store")
    );
    let (status, css) = request(&client, Method::Get, "/assets/styles/style.css", "").await;
    assert_eq!(status, Status::Ok);
    assert!(css.contains(":root"));
    let (_, html) = request(&client, Method::Get, "/companies/new", "").await;
    let token = html
        .split("name=\"csrf\" value=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    assert_eq!(
        request(
            &client,
            Method::Post,
            "/companies",
            &format!("csrf={token}&name=Test&address=Street&contact_no=555")
        )
        .await
        .0,
        Status::SeeOther
    );
    let company = roster_data::companies::list(&pool)
        .await
        .unwrap()
        .pop()
        .unwrap()
        .id;
    assert_eq!(
        request(
            &client,
            Method::Post,
            &format!("/companies/{company}"),
            &format!("csrf={token}&name=Renamed&address=Updated&contact_no=556")
        )
        .await
        .0,
        Status::SeeOther
    );
    let (status, html) = request(
        &client,
        Method::Get,
        &format!("/companies/{company}/edit"),
        "",
    )
    .await;
    assert_eq!(status, Status::Ok);
    assert!(html.contains("value=\"Renamed\""));
    assert_eq!(request(&client, Method::Post, "/employees",
        &format!("csrf={token}&name=Person&contact_no=555&email=test%40example.test&comp_id={company}")).await.0, Status::SeeOther);
    let employee = roster_data::employees::list(&pool)
        .await
        .unwrap()
        .pop()
        .unwrap()
        .id;
    assert_eq!(
        request(
            &client,
            Method::Post,
            &format!("/employees/{employee}"),
            &format!("csrf={token}&name=Updated&contact_no=556&email=new%40example.test&comp_id=1")
        )
        .await
        .0,
        Status::SeeOther
    );
    let (_, html) = request(
        &client,
        Method::Get,
        &format!("/employees/{employee}/edit"),
        "",
    )
    .await;
    assert!(html.contains("value=\"new@example.test\""));
    assert_eq!(
        roster_data::employees::find(&pool, employee)
            .await
            .unwrap()
            .unwrap()
            .comp_id,
        Some(1)
    );
    assert_eq!(
        request(
            &client,
            Method::Post,
            &format!("/employees/{employee}/delete"),
            &format!("csrf={token}")
        )
        .await
        .0,
        Status::SeeOther
    );
    assert_eq!(
        request(
            &client,
            Method::Get,
            &format!("/employees/{employee}/edit"),
            ""
        )
        .await
        .0,
        Status::NotFound
    );
    assert_eq!(
        request(
            &client,
            Method::Post,
            "/companies/1/delete",
            &format!("csrf={token}")
        )
        .await
        .0,
        Status::SeeOther
    );
    assert_eq!(roster_data::employees::list(&pool).await.unwrap().len(), 1);
    assert_eq!(
        request(
            &client,
            Method::Post,
            &format!("/companies/{company}/delete"),
            &format!("csrf={token}")
        )
        .await
        .0,
        Status::SeeOther
    );
    assert_eq!(
        request(
            &client,
            Method::Get,
            &format!("/companies/{company}/edit"),
            ""
        )
        .await
        .0,
        Status::NotFound
    );
}

#[rocket::async_test]
async fn forms_validate_tokens_input_and_html_escaping() {
    let pool = create_pool("sqlite::memory:").await.unwrap();
    let client = Client::tracked(create_router(pool.clone(), assets()))
        .await
        .unwrap();
    let (_, html) = request(&client, Method::Get, "/companies/new", "").await;
    let token = html
        .split("name=\"csrf\" value=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    assert_eq!(
        request(&client, Method::Post, "/companies/1/delete", "csrf=wrong")
            .await
            .0,
        Status::Forbidden
    );
    assert_eq!(
        request(&client, Method::Get, "/companies/1/delete", "")
            .await
            .0,
        Status::NotFound
    );
    let (status, html) = request(
        &client,
        Method::Post,
        "/companies",
        &format!("csrf={token}&name=&address=Preserved&contact_no=555"),
    )
    .await;
    assert_eq!(status, Status::UnprocessableEntity);
    assert!(html.contains("value=\"Preserved\""));
    for selection in ["", "invalid", "9999"] {
        assert_eq!(request(&client, Method::Post, "/employees",
            &format!("csrf={token}&name=Retained&contact_no=555&email=test%40example.test&comp_id={selection}")).await.0,
            Status::UnprocessableEntity);
    }
    assert_eq!(roster_data::dashboard_counts(&pool).await.unwrap(), (2, 2));
    assert_eq!(
        request(
            &client,
            Method::Post,
            "/companies/999",
            &format!("csrf={token}&name=Missing&address=Address&contact_no=555")
        )
        .await
        .0,
        Status::NotFound
    );
    assert_eq!(
        request(
            &client,
            Method::Post,
            "/companies",
            &format!("csrf={token}&name=%3Cscript%3E&address=Street&contact_no=555")
        )
        .await
        .0,
        Status::SeeOther
    );
    let (_, html) = request(&client, Method::Get, "/companies?saved=created", "").await;
    assert!(html.contains("&lt;script&gt;"));
    assert!(!html.contains("<td><script>"));
    assert!(html.contains("Record added."));
}
