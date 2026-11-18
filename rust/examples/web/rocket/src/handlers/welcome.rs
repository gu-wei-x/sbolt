use crate::handlers::{ResponseFormat, TemplateResult};
use rocket::get;

#[get("/")]
pub(crate) async fn welcome(format: ResponseFormat) -> TemplateResult {
    crate::rocket_example_views::render(
        "views/welcome",
        format.into(),
        &mut sbolt::context! {
            name: "Rocket".to_string()
        },
    )
    .into()
}
