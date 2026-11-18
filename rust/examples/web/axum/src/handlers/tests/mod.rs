use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::Request,
    routing::get,
};
use tower::ServiceExt;

#[tokio::test]
async fn welcome_negotiates_dynamic_views() {
    let app = Router::new().route("/", get(welcome));
    for (accept, content_type, expected) in [
        (None, "text/html", "Welcome: Axum!"),
        (Some("text/html"), "text/html", "Welcome: Axum!"),
        (
            Some("application/json"),
            "application/json",
            "\"framework\": \"Axum\"",
        ),
        (Some("text/plain"), "text/plain", "Welcome to Axum!"),
        (
            Some("text/html;q=0, */*;q=0.8"),
            "application/json",
            "\"framework\": \"Axum\"",
        ),
    ] {
        let mut request = Request::builder().uri("/");
        if let Some(accept) = accept {
            request = request.header(header::ACCEPT, accept);
        }
        let response = app
            .clone()
            .oneshot(request.body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response.headers()[header::CONTENT_TYPE]
                .to_str()
                .unwrap()
                .starts_with(content_type)
        );
        assert_eq!(response.headers()[header::VARY], "Accept");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert!(std::str::from_utf8(&body).unwrap().contains(expected));
    }
}

#[tokio::test]
async fn invalid_or_unsupported_accept_defaults_to_html() {
    let app = Router::new().route("/", get(welcome));
    for accept in [
        "application/xml",
        "*/*;q=0",
        "text/html;q=invalid",
        "invalid",
        "application/json;q=1.5",
        "application/xml;q=invalid, application/json",
        "application/xml;q=1.5, application/json",
    ] {
        let request = Request::builder()
            .uri("/")
            .header(header::ACCEPT, accept)
            .body(Body::empty())
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response.headers()[header::CONTENT_TYPE]
                .to_str()
                .unwrap()
                .starts_with("text/html")
        );
        assert_eq!(response.headers()[header::VARY], "Accept");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert!(
            std::str::from_utf8(&body)
                .unwrap()
                .contains("Welcome: Axum!")
        );
    }
}

#[tokio::test]
async fn failed_json_render_is_not_labeled_json() {
    let app = Router::new().route("/missing-json", get(missing_json));
    let request = Request::builder()
        .uri("/missing-json")
        .header(header::ACCEPT, "application/json")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(response.headers()[header::VARY], "Accept");
    assert!(
        response.headers()[header::CONTENT_TYPE]
            .to_str()
            .unwrap()
            .starts_with("text/plain")
    );
}

async fn missing_json(format: ResponseFormat) -> TemplateResult {
    let result =
        crate::axum_example_views::render("views/missing", format.into(), &mut sbolt::context!());
    result.into()
}

#[tokio::test]
async fn kind_extractor_combines_accept_headers() {
    let app = Router::new().route("/", get(welcome));
    let request = Request::builder()
        .uri("/")
        .header(header::ACCEPT, "text/html;q=0")
        .header(header::ACCEPT, "application/json")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");
}
