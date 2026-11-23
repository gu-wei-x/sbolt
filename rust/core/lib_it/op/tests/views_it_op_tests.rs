#![cfg(test)]
use sbolt::types::result;
use sbolt::types::template::Kind;
sbolt::include_views!();

use lib_it_op_views::render as render_custom_view;
use lib_it_op_views::resolve as resolve_custom_view;
#[path = "../../view_tests.rs"]
mod view_tests;

#[test]
fn static_html_compaction_renders_exact_output() -> result::RenderResult<()> {
    let model = view_tests::TypedModel {
        name: "<pre>".into(),
        count: 42,
    };
    let (output, _) = lib_it_op_views::render(
        "views/html_compaction",
        Some(Kind::KHTML),
        &mut sbolt::context!(model),
    )?;
    assert_eq!(
        output,
        "<html><head><title>Welcome</title></head><body><div>&lt;pre&gt;|42</div></body></html>"
    );
    Ok(())
}

#[test]
fn render_selects_kind_and_matching_layout() -> result::RenderResult<()> {
    for (kind, expected) in [
        (Kind::KHTML, "<main><p>html</p></main>"),
        (Kind::KJSON, "{\"wrapped\":{\"format\":\"json\"}\n}"),
        (Kind::KTEXT, "text-layout:text value"),
    ] {
        let (output, selected_kind) =
            lib_it_op_views::render("views/formats/page", Some(kind), &mut sbolt::context!())?;
        assert_eq!(selected_kind, kind);
        assert_eq!(output.trim(), expected);
    }
    Ok(())
}

#[test]
fn single_dynamic_sections_preserve_execution() -> result::RenderResult<()> {
    let (output, _) = lib_it_op_views::render(
        "views/sections_dynamic",
        Some(Kind::KTEXT),
        &mut sbolt::context!(),
    )?;
    assert_eq!(output, "7|7|01|7");
    Ok(())
}

#[test]
fn template_modules_do_not_collide_with_sibling_directories() -> result::RenderResult<()> {
    for (path, expected) in [
        ("views/formats/page", "<main><p>html</p></main>"),
        (
            "views/formats/page_html/child",
            "<p>nested HTML directory</p>",
        ),
        (
            "views/formats/page_html_/other",
            "<p>nested underscored directory</p>",
        ),
    ] {
        let (output, kind) = lib_it_op_views::render(path, None, &mut sbolt::context!())?;
        assert_eq!(kind, Kind::KHTML);
        assert_eq!(output.trim(), expected);
    }
    Ok(())
}

#[test]
fn render_defaults_to_html_without_cross_kind_fallback() -> result::RenderResult<()> {
    let (output, kind) =
        lib_it_op_views::render("views/formats/page", None, &mut sbolt::context!())?;
    assert_eq!(kind, Kind::KHTML);
    assert_eq!(output.trim(), "<main><p>html</p></main>");
    let error =
        lib_it_op_views::render("views/json_static", None, &mut sbolt::context!()).unwrap_err();
    assert!(matches!(
        error,
        sbolt::types::error::RuntimeError::NotFound(..)
    ));
    Ok(())
}

#[test]
fn html_layouts_compact_only_known_insertion_contexts() -> result::RenderResult<()> {
    for (path, default_section, expected) in [
        ("views/formats/page", None, "<main><p>html</p></main>"),
        (
            "views/formats/case_sensitive/page",
            None,
            "<main><section><p>html</p></section></main>",
        ),
        (
            "views/formats/pre_layout",
            Some("<p>html</p>\n"),
            "<pre><p>html</p>\n</pre>\n",
        ),
        (
            "views/formats/inside_pre_page",
            None,
            "<pre><section><p>html</p>\n</section>\n</pre>\n",
        ),
    ] {
        let mut context = sbolt::context!();
        if let Some(content) = default_section {
            sbolt::types::Context::set_default_section(&mut context, content.to_string());
        }
        let (output, _) = lib_it_op_views::render(path, Some(Kind::KHTML), &mut context)?;
        assert_eq!(output, expected, "{path}");
    }
    Ok(())
}

#[test]
fn missing_views_return_render_errors() {
    let result =
        lib_it_op_views::render("views/missing", Some(Kind::KJSON), &mut sbolt::context!());
    assert!(matches!(
        result,
        Err(sbolt::types::error::RuntimeError::NotFound(..))
    ));
}

#[test]
fn render_never_falls_back_to_another_kind() {
    let error = lib_it_op_views::render(
        "views/json_static",
        Some(Kind::KHTML),
        &mut sbolt::context!(),
    )
    .unwrap_err();
    assert!(matches!(
        error,
        sbolt::types::error::RuntimeError::NotFound(..)
    ));
    let error = lib_it_op_views::render(
        "views/formats/missing_layout",
        Some(Kind::KHTML),
        &mut sbolt::context!(),
    )
    .unwrap_err();
    assert!(
        matches!(error, sbolt::types::error::RuntimeError::NotFound(_, message) if message.contains("Layout"))
    );
}

#[test]
fn static_json_preserves_values_and_removes_only_structural_whitespace() -> result::RenderResult<()>
{
    let (output, _) = lib_it_op_views::render(
        "views/json_static",
        Some(Kind::KJSON),
        &mut sbolt::context!(),
    )?;
    assert_eq!(
        output,
        r#"{"z":[true,false,null,{},[],-0,1.2300e+999],"text":" a  b \n \t \" \\ \/ \u0041 ","z":9007199254740993123456789}"#
    );
    Ok(())
}

#[test]
fn dynamic_json_keeps_existing_formatting() -> result::RenderResult<()> {
    let mut context = sbolt::context! {
        name: "sbolt".to_string(),
        age: 1,
        msg: "Welcome!".to_string()
    };
    let (output, _) = lib_it_op_views::render("views/sub/jindex", Some(Kind::KJSON), &mut context)?;
    assert_eq!(
        output.trim(),
        "{\n  \"name\": \"sbolt\",\n  \"age\": 1,\n  \"msg\": \"Welcome!\"}"
    );
    Ok(())
}

#[test]
fn optimized_html_preserves_structure_and_attribute_values() -> result::RenderResult<()> {
    let (result, _) =
        lib_it_op_views::render("views/html_regressions", None, &mut sbolt::context!())?;
    assert_eq!(
        result.trim(),
        r#"<div></div><my-widget>hello</my-widget><p title='say "hi" &amp; goodbye'>after</p>"#
    );
    Ok(())
}

#[test]
fn static_compaction_respects_sections_and_loops() -> result::RenderResult<()> {
    let (result, _) = lib_it_op_views::render("views/boundaries", None, &mut sbolt::context!())?;
    assert!(
        result.starts_with("<div class='x'>ab</div><p>block</p><span>one</span> <span>two</span>")
    );
    assert!(result.contains("<pre><span   title = ' a  b '> hello  world </span></pre>"));
    assert!(result.contains("<span   data-x = 'y'>0 </span><span   data-x = 'y'>1 </span>"));
    Ok(())
}

#[test]
fn default_view() -> result::RenderResult<()> {
    let (result, _) = lib_it_op_views::render("views/default", None, &mut sbolt::context!())?;
    let expected =
        "<html><head><title>Default</title></head><body><div>Hello Default!</div></body></html>";
    assert_eq!(result.trim(), expected);

    Ok(())
}

#[test]
fn sub_index_view() -> result::RenderResult<()> {
    let mut context = sbolt::context! {
        name: "sbolt".to_string(),
        age: 1,
        msg: "Welcome!".to_string()
    };
    let (result, _) = lib_it_op_views::render("views/sub/index", None, &mut context)?;
    let expected = "<html>\n    <head>\n        <title>Welcome</title>\n    </head>\n    <body>\n        <div>Welcome! - from sbolt(1)</div>\n    </body>\n</html>";
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn sub_home_view() -> result::RenderResult<()> {
    let (result, _) = lib_it_op_views::render("views/sub/home", None, &mut sbolt::context!())?;
    let expected = "<html><head>\n        \n    <title>Home</title>\n    </head>\n    <body>\n        <div class=\"menu\">\n           \n    <ul>\n        <li>menu 1</li>\n        <li>menu 2</li>\n        <li>menu 3</li>\n    </ul>\n        </div>\n        <div class=\"main\">\n           <div>Hello world!<div>        </div>\n        <div class=\"footer\">\n           \n    this is footer\n        </div>\n    </body>\n</html>";
    assert_eq!(result, expected);

    Ok(())
}
