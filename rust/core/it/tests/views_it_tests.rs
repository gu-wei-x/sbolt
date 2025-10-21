#![cfg(test)]
use sbolt::types::result;
sbolt::include_views!();

#[test]
#[should_panic]
fn no_existing_view() {
    lib_it_views::render("views/no_existing", &mut sbolt::context!()).unwrap();
}

#[test]
fn comp_test_view() -> result::RenderResult<()> {
    let result = lib_it_views::render("views/default", &mut sbolt::context!())?;
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
fn comp_index_view() -> result::RenderResult<()> {
    let mut context = sbolt::context! {
        name: "sbolt".to_string(),
        age: 1,
        msg: "Welcome!".to_string()
    };
    let result = lib_it_views::render("views/sub/index", &mut context)?;
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
fn comp_home_view() -> result::RenderResult<()> {
    let result = lib_it_views::render("views/sub/home", &mut sbolt::context!())?;
    assert!(result.contains("<title>Home</title>"));
    assert!(result.contains("<li>menu 1</li>"));
    assert!(result.contains("this is footer"));

    Ok(())
}

#[test]
fn comp_jindex_view() -> result::RenderResult<()> {
    let mut context = sbolt::context! {
        name: "sbolt".to_string(),
        age: 1,
        msg: "Welcome!".to_string()
    };
    let result = lib_it_views::render("views/sub/jindex", &mut context)?;
    let expected = "{\n  \"name\": \"sbolt\",\n  \"age\": 1,\n  \"msg\": \"Welcome!\"\n}";
    assert_eq!(result.trim(), expected);

    Ok(())
}
