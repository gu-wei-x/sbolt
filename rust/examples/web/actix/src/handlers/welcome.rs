use crate::handlers::{ResponseFormat, TemplateResult};
use example_views::models::WelcomeModel;

pub(crate) async fn welcome(format: ResponseFormat) -> TemplateResult {
    example_views::render(
        "views/welcome",
        format.into(),
        &mut sbolt::context!(WelcomeModel {
            name: "Actix Web".into(),
        }),
    )
    .into()
}
