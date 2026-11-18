use crate::handlers::{ResponseFormat, TemplateResult};

pub(crate) async fn welcome(format: ResponseFormat) -> TemplateResult {
    crate::axum_example_views::render(
        "views/welcome",
        format.into(),
        &mut sbolt::context! {
            name: "Axum".to_string()
        },
    )
    .into()
}
