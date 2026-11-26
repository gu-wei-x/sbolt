use sbolt::types::{Context, result::RenderResult, template::Kind};

pub mod models;

sbolt::include_views!();

pub fn render(
    name: &str,
    kind: Option<Kind>,
    context: &mut impl Context,
) -> RenderResult<(String, Kind)> {
    shared_views::render(name, kind, context)
}
