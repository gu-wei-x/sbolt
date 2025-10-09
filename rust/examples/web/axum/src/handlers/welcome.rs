use crate::handlers::TemplateResult;

pub(crate) async fn welcome() -> TemplateResult {
    crate::axum_example_views::render(
        "views/welcome",
        &mut disguise::context! {
            name: "Axum".to_string()
        },
    )
    .into()
}
