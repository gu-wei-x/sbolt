use crate::handlers::TemplateResult;

pub(crate) async fn welcome() -> TemplateResult {
    crate::actix_web_example_views::render(
        "views/welcome",
        &mut disguise::context! {
            name: "Actix Web".to_string()
        },
    )
    .into()
}
