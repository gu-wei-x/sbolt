use super::*;
use actix_web::{
    App,
    http::{StatusCode, header},
    test, web,
};

#[actix_web::test]
async fn welcome_negotiates_dynamic_views() {
    let app = test::init_service(App::new().route("/", web::get().to(welcome))).await;
    for (accept, content_type, expected) in [
        (None, "text/html", "Welcome: Actix Web!"),
        (Some("text/html"), "text/html", "Welcome: Actix Web!"),
        (
            Some("application/json"),
            "application/json",
            "\"framework\": \"Actix Web\"",
        ),
        (Some("text/plain"), "text/plain", "Welcome to Actix Web!"),
        (
            Some("text/html;q=0, */*;q=0.8"),
            "application/json",
            "\"framework\": \"Actix Web\"",
        ),
    ] {
        let mut request = test::TestRequest::get().uri("/");
        if let Some(accept) = accept {
            request = request.insert_header((header::ACCEPT, accept));
        }
        let response = test::call_service(&app, request.to_request()).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response
                .headers()
                .get(header::CONTENT_TYPE)
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(content_type)
        );
        assert_eq!(response.headers().get(header::VARY).unwrap(), "Accept");
        let body = test::read_body(response).await;
        assert!(std::str::from_utf8(&body).unwrap().contains(expected));
    }
}

#[actix_web::test]
async fn invalid_or_unsupported_accept_defaults_to_html() {
    let app = test::init_service(App::new().route("/", web::get().to(welcome))).await;
    for accept in [
        "application/xml",
        "*/*;q=0",
        "text/html;q=invalid",
        "invalid",
        "application/json;q=1.5",
        "application/xml;q=invalid, application/json",
        "application/xml;q=1.5, application/json",
    ] {
        let request = test::TestRequest::get()
            .uri("/")
            .insert_header((header::ACCEPT, accept))
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response
                .headers()
                .get(header::CONTENT_TYPE)
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("text/html")
        );
        assert_eq!(response.headers().get(header::VARY).unwrap(), "Accept");
        let body = test::read_body(response).await;
        assert!(
            std::str::from_utf8(&body)
                .unwrap()
                .contains("Welcome: Actix Web!")
        );
    }
}

#[actix_web::test]
async fn failed_json_render_is_not_labeled_json() {
    let app =
        test::init_service(App::new().route("/missing-json", web::get().to(missing_json))).await;
    let request = test::TestRequest::get()
        .uri("/missing-json")
        .insert_header((header::ACCEPT, "application/json"))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(response.headers().get(header::VARY).unwrap(), "Accept");
    assert!(
        response
            .headers()
            .get(header::CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("text/plain")
    );
}

async fn missing_json(format: ResponseFormat) -> TemplateResult {
    let result = example_views::render("views/missing", format.into(), &mut sbolt::context!());
    result.into()
}

#[actix_web::test]
async fn kind_extractor_combines_accept_headers() {
    let app = test::init_service(App::new().route("/", web::get().to(welcome))).await;
    let request = test::TestRequest::get()
        .uri("/")
        .append_header((header::ACCEPT, "text/html;q=0"))
        .append_header((header::ACCEPT, "application/json"))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE).unwrap(),
        "application/json"
    );
}
