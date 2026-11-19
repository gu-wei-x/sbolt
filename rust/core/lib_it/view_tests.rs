use super::render_custom_view as render;
use sbolt::types::{error::RuntimeError, result::RenderResult, template::Kind};

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
