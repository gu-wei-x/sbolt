#![deny(warnings)]

include!(env!("NO_LAYOUT_TEMPLATES_FILES"));

pub struct Model {
    pub name: String,
}

#[test]
fn imported_models_render_without_layouts_under_denied_warnings()
-> sbolt::types::result::RenderResult<()> {
    let model = std::sync::Arc::new(Model {
        name: "local model".into(),
    });
    let mut context = sbolt::context!(model: std::sync::Arc::clone(&model));
    let named = no_layout_views::render("views_no_layout/page", None, &mut context)?.0;
    let direct = no_layout_views::resolve("views_no_layout/page", None)?.render(&mut context)?;
    assert_eq!(named, direct);
    assert_eq!(named.trim(), "<p>local model</p>");
    Ok(())
}
