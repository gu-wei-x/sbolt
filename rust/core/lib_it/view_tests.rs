use super::render_custom_view as render;
use super::resolve_custom_view as resolve;
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
        assert_eq!(resolve(path, Some(kind))?.render(&mut context)?, output);
        assert_eq!(selected_kind, kind);
        let expected_name = match kind {
            Kind::KHTML => "&lt;b&gt;&amp;team&lt;/b&gt;",
            Kind::KJSON => "\\u003cb\\u003e\\u0026team\\u003c/b\\u003e",
            Kind::KTEXT => "<b>&team</b>",
        };
        assert!(output.contains(expected_name));
        assert!(output.contains("-42"));
    }
    assert_eq!(Arc::strong_count(&model), 3);
    assert_eq!(model.name, "<b>&team</b>");
    Ok(())
}

#[test]
fn whole_json_models_serialize_without_unwrapping() -> RenderResult<()> {
    let mut context = sbolt::context!(model: Arc::new("<b>&model</b>".to_string()));
    let output = resolve("views/json_model", Some(Kind::KJSON))?.render(&mut context)?;
    assert_eq!(output.trim(), r#""\u003cb\u003e\u0026model\u003c/b\u003e""#);
    Ok(())
}

#[test]
fn json_interpolation_serializes_complete_values_and_preserves_fragments() -> RenderResult<()> {
    let mut context = sbolt::context!(
        message: "</script>&'\"\n\u{2028}\u{2029}".to_string(),
        fragment: "{\"trusted\":\"</script>\"}".to_string()
    );
    context.set_capacity_hint(4096);
    let output = resolve("views/json_values", Some(Kind::KJSON))?.render(&mut context)?;
    assert_eq!(
        output.trim(),
        r#"{
  "message": "\u003c/script\u003e\u0026\u0027\"\n\u2028\u2029",
  "number": -42,
  "float": 1.25,
  "boolean": true,
  "none": null,
  "some": 4,
  "array": [1,2],
  "object": {"key":3},
  "json_string": "{\"nested\":true}",
  "fragment": {"trusted":"\u003c/script\u003e"},
  "evaluated_once": 1,
  "evaluations": 1,
  "sequence": [1,2]
}"#
    );
    assert!(output.capacity() >= 4096);
    Ok(())
}

#[test]
fn json_interpolation_propagates_serialization_and_fragment_errors() -> RenderResult<()> {
    let handle = resolve("views/json_values", Some(Kind::KJSON))?;
    let error = handle.render(&mut sbolt::context!(mode: 1u8)).unwrap_err();
    assert!(matches!(
        error,
        RuntimeError::InvalidInput(ref message, ref detail)
            if message == "JSON serialization failed" && detail.contains("key must be a string")
    ));
    let error = handle
        .render(&mut sbolt::context!(
            message: String::new(),
            fragment: "{\"broken\":".to_string()
        ))
        .unwrap_err();
    assert!(matches!(
        error,
        RuntimeError::InvalidInput(ref message, _) if message == "Invalid raw JSON"
    ));
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
            let handle = resolve("views/typed", Some(kind)).unwrap();
            assert!(matches!(
                handle.render(&mut context),
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
fn standard_formatting_and_early_returns_preserve_rendering_contracts() -> RenderResult<()> {
    let handle = resolve("views/render_regressions", None)?;
    for (mode, expected) in [
        (0u8, "before|value: 42"),
        (1, "early"),
        (2, "before|"),
        (4, "before|&lt;em&gt;42&lt;/em&gt;|1.25|true|-42"),
        (6, "before|specialized"),
        (
            7,
            "before|<p>1:&lt;em&gt;1&lt;/em&gt;</p><p>2:&lt;em&gt;2&lt;/em&gt;</p>",
        ),
    ] {
        let actual = handle.render(&mut sbolt::context!(mode: mode))?;
        assert_eq!(actual.trim(), expected);
        let mut context = sbolt::context!(mode: mode);
        context.set_capacity_hint(4096);
        let hinted = handle.render(&mut context)?;
        assert_eq!(hinted, actual);
        if mode != 1 {
            assert!(hinted.capacity() >= 4096);
        }
    }
    assert!(matches!(
        handle.render(&mut sbolt::context!(mode: 3u8)),
        Err(RuntimeError::InvalidInput(_, _))
    ));
    Ok(())
}

#[test]
fn borrowed_raw_values_bypass_only_their_own_escaping() -> RenderResult<()> {
    let mut context = sbolt::context!(
        mode: 8u8,
        markup: sbolt::utils::raw("<strong>context</strong>")
    );
    let output = resolve("views/render_regressions", None)?.render(&mut context)?;
    assert_eq!(
        output.trim(),
        "before|<b>&\"trusted\"</b>|<b>&\"trusted\"</b>|<b>&\"trusted\"</b>|<i>mutable</i>|<strong>context</strong>|&lt;u&gt;escaped&lt;/u&gt;"
    );
    Ok(())
}

#[test]
fn section_capacity_hints_exclude_other_output_streams() -> RenderResult<()> {
    let mut context = sbolt::context!();
    context.set_capacity_hint(4096);
    let (output, _) = render("views/section_capacity", None, &mut context)?;
    assert!(output.len() > 500);
    assert!(output.capacity() >= 4096);
    for (name, expected, count) in [("tiny", "x", 3), ("outer", "ab", 1), ("inner", "nested", 1)] {
        let sections = context.get_section(name).unwrap();
        assert_eq!(sections.len(), count);
        for section in sections {
            assert_eq!(section, expected);
            assert!(
                section.capacity() <= 16,
                "{name}: capacity={}",
                section.capacity()
            );
        }
    }
    Ok(())
}

#[test]
fn typed_model_rendering_keeps_models_and_sections_independent() -> RenderResult<()> {
    let model = Arc::new(TypedModel {
        name: "local".into(),
        count: 7,
    });
    let mut context = sbolt::context!(
        model: Arc::clone(&model),
        page: Arc::clone(&model),
        layout: Arc::new(LayoutModel { title: "independent layout".into() })
    );
    context.set_capacity_hint(1024);
    let handle = resolve("views/typed", Some(Kind::KTEXT))?;
    let output = handle.render(&mut context)?;
    assert_eq!(output, "local|7|local");
    assert!(output.capacity() >= 1024);
    let html = resolve("views/typed_layout", None)?.render(&mut context)?;
    assert!(html.contains("<main>independent layout|"));
    assert!(html.contains("<p>local:7</p>"));
    assert_eq!(
        context.get_section("caption").unwrap().last().unwrap(),
        "local"
    );
    assert!(html.capacity() >= 1024);
    assert_eq!(model.name, "local");
    assert_eq!(Arc::strong_count(&model), 3);
    Ok(())
}

#[test]
fn context_capacity_hints_preserve_output_with_and_without_layouts() -> RenderResult<()> {
    for (path, kind) in [
        ("views/custom/value", Kind::KTEXT),
        ("views/formats/page", Kind::KHTML),
        ("views/formats/page", Kind::KJSON),
        ("views/formats/page", Kind::KTEXT),
    ] {
        let handle = resolve(path, Some(kind))?;
        let hinted_context = || {
            let mut context = sbolt::context!(count: 3i32);
            context.set_capacity_hint(4096);
            context
        };
        let unhinted = render(path, Some(kind), &mut sbolt::context!(count: 3i32))?.0;
        let (named, actual_kind) = render(path, Some(kind), &mut hinted_context())?;
        assert_eq!(actual_kind, kind);
        assert_eq!(named, unhinted);
        assert!(named.capacity() >= 4096);
        let expected = handle.render(&mut hinted_context())?;
        assert_eq!(expected, named);
        assert!(expected.capacity() >= 4096);
        let mut context = hinted_context();
        context.set_capacity_hint(0);
        let reset = handle.render(&mut context)?;
        assert_eq!(reset, expected);
        assert!(reset.capacity() < 4096);
    }
    Ok(())
}

#[test]
fn rendering_propagates_errors_without_claiming_success() -> RenderResult<()> {
    let handle = resolve("views/sections", Some(Kind::KTEXT))?;
    let error = handle.render(&mut sbolt::context!()).unwrap_err();
    assert!(matches!(error, RuntimeError::NotFound(_, _)));
    Ok(())
}

#[test]
fn resolved_templates_preserve_kind_layouts_and_reborrow_contexts() -> RenderResult<()> {
    for kind in [Kind::KHTML, Kind::KJSON, Kind::KTEXT] {
        let handle = resolve("/views/formats/page/", Some(kind))?;
        for _ in 0..2 {
            let actual = handle.render(&mut sbolt::context!())?;
            let (expected, selected_kind) =
                render("views/formats/page", Some(kind), &mut sbolt::context!())?;
            assert_eq!(selected_kind, kind);
            assert_eq!(actual, expected);
        }
    }
    let handle = resolve("views/custom/value", Some(Kind::KTEXT))?;
    for count in [1i32, 99] {
        assert_eq!(
            handle.render(&mut sbolt::context!(count: count))?,
            format!("[{count}]\n")
        );
    }
    Ok(())
}

#[test]
fn resolution_reports_missing_paths_and_kinds() {
    for (path, kind) in [
        ("", None),
        ("views/nonexistent", None),
        ("views/json_static", Some(Kind::KTEXT)),
    ] {
        assert!(matches!(
            resolve(path, kind),
            Err(RuntimeError::NotFound(_, _))
        ));
    }
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
