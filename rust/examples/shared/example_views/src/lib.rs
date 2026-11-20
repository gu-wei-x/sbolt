use sbolt::types::{Context, result::RenderResult, template::Kind};

// Keep model paths identical for shared and directly compiled templates.
extern crate self as example_views;

pub mod models;

sbolt::include_views!();

// The generated renderer is pub(crate), so a wrapper exposes it to other crates.
pub fn render(
    name: &str,
    kind: Option<Kind>,
    context: &mut impl Context,
) -> RenderResult<(String, Kind)> {
    shared_views::render(name, kind, context)
}
