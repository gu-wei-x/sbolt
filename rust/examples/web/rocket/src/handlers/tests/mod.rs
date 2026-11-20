use super::*;
use rocket::local::blocking::Client;

#[test]
fn accept_negotiation_selects_supported_representations() {
    let client = Client::tracked(rocket::build().mount("/", routes())).unwrap();
    for (accept, content_type, expected_body) in [
        (
            "application/json",
            ContentType::JSON,
            "\"framework\": \"Rocket\"",
        ),
        (
            "text/plain",
            ContentType::Plain,
            "This is a plain-text response.",
        ),
        ("text/html", ContentType::HTML, "Welcome: Rocket!"),
        (
            "application/json;q=0.9, text/html;q=0.1",
            ContentType::JSON,
            "\"framework\": \"Rocket\"",
        ),
        (
            "application/xml, application/json;q=0.5",
            ContentType::JSON,
            "\"framework\": \"Rocket\"",
        ),
        ("text/*", ContentType::HTML, "Welcome: Rocket!"),
        (
            "application/*",
            ContentType::JSON,
            "\"framework\": \"Rocket\"",
        ),
        ("*/*", ContentType::HTML, "Welcome: Rocket!"),
        (
            "text/html;q=0, */*;q=0.7",
            ContentType::JSON,
            "\"framework\": \"Rocket\"",
        ),
        (
            "application/json;q=0, */*;q=0.8",
            ContentType::HTML,
            "Welcome: Rocket!",
        ),
        (
            "application/json, text/html",
            ContentType::JSON,
            "\"framework\": \"Rocket\"",
        ),
        (
            "text/html;level=1, application/json;q=0.5",
            ContentType::JSON,
            "\"framework\": \"Rocket\"",
        ),
        (
            "text/html;charset=utf-8;q=0, application/json;q=0.5",
            ContentType::JSON,
            "\"framework\": \"Rocket\"",
        ),
    ] {
        let response = client
            .get("/")
            .header(Header::new("Accept", accept))
            .dispatch();
        assert_eq!(response.status(), Status::Ok, "{accept}");
        assert_eq!(response.content_type(), Some(content_type), "{accept}");
        assert_eq!(response.headers().get_one("Vary"), Some("Accept"));
        assert!(
            response.into_string().unwrap().contains(expected_body),
            "{accept}"
        );
    }
}

#[test]
fn unsupported_preferences_default_to_html() {
    let client = Client::tracked(rocket::build().mount("/", routes())).unwrap();
    for accept in [
        "application/xml",
        "*/*;q=0",
        "text/html;q=0, application/json;q=0, text/plain;q=0, */*;q=1",
    ] {
        let response = client
            .get("/")
            .header(Header::new("Accept", accept))
            .dispatch();
        assert_eq!(response.status(), Status::Ok, "{accept}");
        assert_eq!(response.content_type(), Some(ContentType::HTML));
        assert_eq!(response.headers().get_one("Vary"), Some("Accept"));
        assert!(response.into_string().unwrap().contains("Welcome: Rocket!"));
    }
}

#[test]
fn request_content_type_does_not_select_response_format() {
    let client = Client::tracked(rocket::build().mount("/", routes())).unwrap();
    let response = client.get("/").header(ContentType::JSON).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
}

#[test]
fn text_view_returns_plain_text_content_type_and_body() {
    let client = Client::tracked(rocket::build().mount("/", routes())).unwrap();
    let response = client
        .get("/")
        .header(Header::new("Accept", "text/plain"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(
        response.into_string().unwrap(),
        "Welcome to Rocket!\nThis is a plain-text response.\n"
    );
}

#[test]
fn json_view_returns_json_content_type_and_body() {
    let client = Client::tracked(rocket::build().mount("/", routes())).unwrap();
    let response = client
        .get("/")
        .header(Header::new("Accept", "application/json"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(
        response.into_string().unwrap().trim(),
        "{\n  \"framework\": \"Rocket\",\n  \"message\": \"Welcome!\"\n}"
    );
}

#[test]
fn json_and_text_views_render_model_fields() {
    for name in ["Axum", "Actix"] {
        for (kind, expected) in [
            (
                Kind::KJSON,
                format!("{{\n  \"framework\": \"{name}\",\n  \"message\": \"Welcome!\"\n}}"),
            ),
            (
                Kind::KTEXT,
                format!("Welcome to {name}!\nThis is a plain-text response."),
            ),
        ] {
            let (output, selected_kind) = example_views::render(
                "views/welcome",
                Some(kind),
                &mut sbolt::context!(example_views::models::WelcomeModel { name: name.into() }),
            )
            .unwrap();
            assert_eq!(selected_kind, kind);
            assert_eq!(output.trim(), expected);
        }
    }
}

#[test]
fn welcome_view_still_returns_html() {
    let client = Client::tracked(rocket::build().mount("/", routes())).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert!(response.into_string().unwrap().contains("Welcome: Rocket!"));
}

#[rocket::get("/missing-json")]
fn missing_json(format: ResponseFormat) -> TemplateResult {
    let result = example_views::render("views/missing", format.into(), &mut sbolt::context!());
    result.into()
}

#[test]
fn failed_json_render_returns_internal_server_error() {
    let client =
        Client::tracked(rocket::build().mount("/", rocket::routes![missing_json])).unwrap();
    let response = client
        .get("/missing-json")
        .header(Header::new("Accept", "application/json"))
        .dispatch();
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.headers().get_one("Vary"), Some("Accept"));
    assert_ne!(response.content_type(), Some(ContentType::JSON));
    assert!(response.into_string().unwrap().contains("views/missing"));
}

#[test]
fn malformed_preferences_default_to_html() {
    let client = Client::tracked(rocket::build().mount("/", routes())).unwrap();
    for accept in [
        "invalid",
        "text/html;q=invalid",
        "application/json;q=1.5",
        "application/xml;q=invalid, application/json",
        "application/xml;q=1.5, application/json",
    ] {
        let response = client
            .get("/")
            .header(Header::new("Accept", accept))
            .dispatch();
        assert_eq!(response.status(), Status::Ok, "{accept}");
        assert_eq!(response.content_type(), Some(ContentType::HTML));
        assert!(response.into_string().unwrap().contains("Welcome: Rocket!"));
    }
}

#[test]
fn kind_guard_combines_accept_headers() {
    let client = Client::tracked(rocket::build().mount("/", routes())).unwrap();
    let response = client
        .get("/")
        .header(Header::new("Accept", "text/html;q=0"))
        .header(Header::new("Accept", "application/json"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}
