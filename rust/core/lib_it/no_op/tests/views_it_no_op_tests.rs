#![cfg(test)]
use sbolt::types::result;
use sbolt::types::template::Kind;
sbolt::include_views!();

#[test]
fn render_selects_shared_names_without_optimization() -> result::RenderResult<()> {
    for (kind, expected) in [
        (Kind::KHTML, "<main><p>html</p>\n</main>"),
        (Kind::KJSON, "{\"wrapped\":{\"format\":\"json\"}\n}"),
        (Kind::KTEXT, "text-layout:text value"),
    ] {
        let (output, selected_kind) =
            lib_it_no_op_views::render("views/formats/page", Some(kind), &mut sbolt::context!())?;
        assert_eq!(selected_kind, kind);
        assert_eq!(output.trim(), expected);
    }
    Ok(())
}

#[test]
fn static_json_preserves_original_formatting() -> result::RenderResult<()> {
    let (output, _) = lib_it_no_op_views::render(
        "views/json_static",
        Some(Kind::KJSON),
        &mut sbolt::context!(),
    )?;
    assert_eq!(output, include_str!("../../views/json_static.rsjson"));
    Ok(())
}

#[test]
fn optional_and_repeated_sections() -> result::RenderResult<()> {
    use sbolt::types::Context;

    let mut context = sbolt::context!();
    context.add_section("required", "required".into());
    let (result, _) =
        lib_it_no_op_views::render("views/sections", Some(Kind::KTEXT), &mut context)?;
    assert_eq!(result.trim(), "required");

    context.add_section("optional", "first".into());
    context.add_section("optional", "second".into());
    let (result, _) =
        lib_it_no_op_views::render("views/sections", Some(Kind::KTEXT), &mut context)?;
    assert_eq!(result.trim(), "firstsecondrequired");
    Ok(())
}

#[test]
fn missing_required_section_returns_error() {
    let error =
        lib_it_no_op_views::render("views/sections", Some(Kind::KTEXT), &mut sbolt::context!())
            .unwrap_err();
    assert!(
        matches!(error, sbolt::types::error::RuntimeError::NotFound(message, _) if message.contains("required"))
    );
}

#[test]
#[should_panic]
fn no_existing_view() {
    lib_it_no_op_views::render("views/no_existing", None, &mut sbolt::context!()).unwrap();
}

#[test]
fn default_view() -> result::RenderResult<()> {
    let (result, _) = lib_it_no_op_views::render("views/default", None, &mut sbolt::context!())?;
    let expected = r#"
<html>
    <head>
        <title>Default</title>
    </head>
    <body>
        <div>Hello Default!</div>
    </body>
</html>"#
        .trim();
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
    let (result, _) = lib_it_no_op_views::render("views/sub/index", None, &mut context)?;
    let expected = r#"
<html>
    <head>
        <title>Welcome</title>
    </head>
    <body>
        <div>Welcome! - from sbolt(1)</div>
    </body>
</html>"#
        .trim();
    assert_eq!(result.trim(), expected);

    Ok(())
}

#[test]
fn sub_home_view() -> result::RenderResult<()> {
    let (result, _) = lib_it_no_op_views::render("views/sub/home", None, &mut sbolt::context!())?;
    assert!(result.contains("<title>Home</title>"));
    assert!(result.contains("<li>menu 1</li>"));
    assert!(result.contains("this is footer"));

    Ok(())
}

#[test]
fn sub_jindex_view() -> result::RenderResult<()> {
    let mut context = sbolt::context! {
        name: "sbolt".to_string(),
        age: 1,
        msg: "Welcome!".to_string()
    };
    let (result, _) =
        lib_it_no_op_views::render("views/sub/jindex", Some(Kind::KJSON), &mut context)?;
    let expected = "{\n  \"name\": \"sbolt\",\n  \"age\": 1,\n  \"msg\": \"Welcome!\"\n}";
    assert_eq!(result.trim(), expected);

    Ok(())
}
