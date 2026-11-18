use crate::handlers::{ResponseFormat, TemplateResult};

pub(crate) async fn welcome(format: ResponseFormat) -> TemplateResult {
    crate::actix_web_example_views::render(
        "views/welcome",
        format.into(),
        &mut sbolt::context! {
            name: "Actix Web".to_string()
        },
    )
    .into()
}
