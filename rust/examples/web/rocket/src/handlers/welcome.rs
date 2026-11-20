use crate::handlers::{ResponseFormat, TemplateResult};
use example_views::models::WelcomeModel;
use rocket::get;

#[get("/")]
pub(crate) async fn welcome(format: ResponseFormat) -> TemplateResult {
    example_views::render(
        "views/welcome",
        format.into(),
        &mut sbolt::context!(WelcomeModel {
            name: "Rocket".into(),
        }),
    )
    .into()
}
