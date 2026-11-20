use super::render_custom_view as render;
use sbolt::types::Context;
use sbolt::types::{error::RuntimeError, result::RenderResult, template::Kind};
use std::sync::Arc;

pub struct TypedModel {
    pub name: String,
    pub count: i32,
}

pub struct LayoutModel {
    pub title: String,
}

#[test]
fn model_directives_render_through_the_named_api() -> RenderResult<()> {
    let model = Arc::new(TypedModel {
        name: "<b>&team</b>".into(),
        count: -42,
    });
    let layout = Arc::new(LayoutModel {
        title: "Shared layout".into(),
    });
    let mut context = sbolt::context!(
        model: Arc::clone(&model),
        page: Arc::clone(&model),
        layout: Arc::clone(&layout)
    );
    for (path, kind) in [
        ("views/typed", Kind::KTEXT),
        ("views/typed", Kind::KJSON),
        ("views/typed_layout", Kind::KHTML),
    ] {
        let (output, selected_kind) = render(path, Some(kind), &mut context)?;
        assert_eq!(selected_kind, kind);
        assert!(output.contains("<b>&team</b>"));
        assert!(output.contains("-42"));
    }
    assert_eq!(Arc::strong_count(&model), 3);
    assert_eq!(model.name, "<b>&team</b>");
    Ok(())
}

#[test]
fn trait_object_models_render_through_the_named_api() -> RenderResult<()> {
    let model: Arc<dyn std::fmt::Display + Send + Sync> = Arc::new(String::from("display model"));
    let (output, kind) = render(
        "views/typed_display",
        Some(Kind::KTEXT),
        &mut sbolt::context!(model: model),
    )?;
    assert_eq!(output.trim(), "display model");
    assert_eq!(kind, Kind::KTEXT);
    Ok(())
}

#[test]
fn layouts_resolve_their_own_models() -> RenderResult<()> {
    let mut context = sbolt::context!(
        model: Arc::new(TypedModel { name: "stored".into(), count: 99 }),
        layout: Arc::new(LayoutModel { title: "layout title".into() })
    );
    let output = render("views/typed_layout", None, &mut context)?.0;
    assert!(output.contains("<main>layout title|"));
    assert!(output.contains("<p>stored:99</p>"));
    assert_eq!(
        context.get_section("caption").unwrap().last().unwrap(),
        "stored"
    );
    let plain = render("views/plain_to_typed", None, &mut context)?.0;
    assert!(plain.contains("<main>layout title|"));
    assert!(plain.contains("<p>plain</p>"));
    Ok(())
}

#[test]
fn layout_models_allow_different_types_but_require_their_declared_keys() -> RenderResult<()> {
    let model = Arc::new(TypedModel {
        name: "passed".into(),
        count: 7,
    });
    for mut context in [
        sbolt::context!(),
        sbolt::context!(layout: Arc::new("wrong key".to_string())),
        sbolt::context!(layout_text: Arc::new(LayoutModel { title: "wrong type".into() })),
    ] {
        context.set_data("model", || Arc::clone(&model));
        let error = render("views/typed_wrong_parent", None, &mut context).unwrap_err();
        assert!(
            matches!(error, RuntimeError::InvalidInput(ref message, ref detail)
            if message.contains("layout_text") && detail.contains("layout_text")),
            "{error:?}"
        );
    }
    let output = render(
        "views/typed_wrong_parent",
        None,
        &mut sbolt::context!(
            model: Arc::clone(&model),
            layout_text: Arc::new("separate layout model".to_string())
        ),
    )?
    .0;
    assert!(output.contains("<main>separate layout model|"));
    assert!(output.contains("<p>passed</p>"));
    Ok(())
}

#[test]
fn named_typed_rendering_reports_missing_or_incorrect_models() {
    for (kind, key) in [(Kind::KTEXT, "page"), (Kind::KJSON, "model")] {
        let mut wrong_type = sbolt::context!();
        wrong_type.set_data(key, || "wrong type".to_string());
        for mut context in [sbolt::context!(), wrong_type] {
            assert!(matches!(
                render("views/typed", Some(kind), &mut context),
                Err(RuntimeError::InvalidInput(ref message, _)) if message.contains(key)
            ));
        }
    }
}

#[test]
fn custom_views_resolve_their_own_model_keys() -> RenderResult<()> {
    let mut context = sbolt::context!(
        model: Arc::new(TypedModel { name: "parent".into(), count: 1 }),
        page: Arc::new(TypedModel { name: "child".into(), count: 7 })
    );
    let output = render("views/custom/typed", Some(Kind::KTEXT), &mut context)?.0;
    assert_eq!(output.trim(), "parent|child|7|child");
    Ok(())
}

#[test]
fn custom_view_inherits_kind_and_renders_child_layout_inline() -> RenderResult<()> {
    for (kind, prefix, suffix) in [
        (Kind::KHTML, "<article>", "</article>\n"),
        (Kind::KJSON, "{\"child\":", "}\n"),
        (Kind::KTEXT, "before:", ":after\n"),
    ] {
        let (child, _) = render("views/formats/page", Some(kind), &mut sbolt::context!())?;
        let (output, selected_kind) =
            render("views/custom/page", Some(kind), &mut sbolt::context!())?;
        assert_eq!(selected_kind, kind);
        assert_eq!(output, format!("{prefix}{child}{suffix}"));
    }
    Ok(())
}

#[test]
fn custom_view_kind_override_selects_child_and_layout_without_changing_parent() -> RenderResult<()>
{
    let (json_child, _) = render(
        "views/formats/page",
        Some(Kind::KJSON),
        &mut sbolt::context!(),
    )?;
    for kind in [Kind::KHTML, Kind::KJSON, Kind::KTEXT] {
        let (child, _) = render("views/formats/page", Some(kind), &mut sbolt::context!())?;
        let (output, selected_kind) = render(
            "views/custom/kind_override",
            Some(Kind::KHTML),
            &mut sbolt::context!(target: "views/formats/page".to_string(), kind: kind),
        )?;
        assert_eq!(selected_kind, Kind::KHTML);
        assert_eq!(output, format!("<section>{json_child}|{child}</section>\n"));
    }
    Ok(())
}

#[test]
fn custom_view_evaluates_arguments_once_in_source_order() -> RenderResult<()> {
    let (output, _) = render(
        "views/custom/argument_order",
        Some(Kind::KHTML),
        &mut sbolt::context!(),
    )?;
    assert_eq!(output, "[9]\nnck");
    Ok(())
}

#[test]
fn custom_view_kind_override_reports_missing_view_or_kind_without_fallback() {
    for target in [
        "views/nonexistent",
        "views/custom/value",
        "views/formats/missing_layout",
    ] {
        let error = render(
            "views/custom/kind_override",
            Some(Kind::KHTML),
            &mut sbolt::context!(target: target.to_string(), kind: Kind::KJSON),
        )
        .unwrap_err();
        assert!(
            matches!(error, RuntimeError::NotFound(ref name, _) if name == target),
            "{error:?}"
        );
    }
}

#[test]
fn custom_view_uses_dynamic_name_and_explicit_isolated_context() -> RenderResult<()> {
    let mut context = sbolt::context!(
        target: "views/custom/card".to_string(),
        secret: "parent only".to_string()
    );
    let (output, _) = render("views/custom/properties", Some(Kind::KTEXT), &mut context)?;
    assert_eq!(output.trim(), "[<a), @b>|true|]\n|parent|");
    Ok(())
}

#[test]
fn custom_views_work_in_sections_and_rust_loops() -> RenderResult<()> {
    let (output, _) = render(
        "views/custom/loop",
        Some(Kind::KTEXT),
        &mut sbolt::context!(),
    )?;
    assert_eq!(output, "[9]\n[0]\n[1]\n");
    Ok(())
}

#[test]
fn custom_view_reborrows_parent_and_explicit_child_contexts() -> RenderResult<()> {
    let mut context = sbolt::context!(count: 5i32);
    let (output, _) = render("views/custom/borrowed", Some(Kind::KTEXT), &mut context)?;
    assert_eq!(output, "[5]\n[5]\n[7]\n[7]\n[7]\n");
    Ok(())
}

#[test]
fn custom_view_accepts_macro_tokens_at_literal_boundaries() -> RenderResult<()> {
    let (output, _) = render(
        "views/custom/lexical",
        Some(Kind::KTEXT),
        &mut sbolt::context!(),
    )?;
    assert_eq!(output, "[9]\n".repeat(8));
    Ok(())
}

#[test]
fn custom_view_reports_missing_view_and_does_not_fall_back_to_other_kinds() {
    for target in ["views/nonexistent", "views/json_static"] {
        let error = render(
            "views/custom/dispatch",
            Some(Kind::KTEXT),
            &mut sbolt::context!(target: target.to_string()),
        )
        .unwrap_err();
        assert!(
            matches!(error, RuntimeError::NotFound(ref name, _) if name == target),
            "{error:?}"
        );
    }
}

#[test]
fn custom_view_propagates_child_render_errors() {
    for (kind, target, expected) in [
        (Kind::KTEXT, "views/sections", "required"),
        (Kind::KHTML, "views/formats/missing_layout", "Layout"),
    ] {
        let error = render(
            "views/custom/dispatch",
            Some(kind),
            &mut sbolt::context!(target: target.to_string()),
        )
        .unwrap_err();
        assert!(
            matches!(error, RuntimeError::NotFound(ref name, ref message)
                if name.contains(expected) || message.contains(expected)),
            "{error:?}"
        );
    }
}
