use crate::handlers::TemplateResult;
use rocket::get;

#[get("/")]
pub(crate) async fn welcome() -> TemplateResult {
    crate::rocket_example_views::render(
        "views/welcome",
        &mut disguise::context! {
            name: "Rocket".to_string()
        },
    )
    .into()
}
