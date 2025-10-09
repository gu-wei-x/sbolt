#![cfg(test)]
use disguise::types::result;
disguise::include_views!();

#[test]
#[should_panic]
fn no_existing_view() {
    lib_it_views::render("views/no_existing", &mut disguise::context!()).unwrap();
}

#[test]
fn comp_test_view() -> result::RenderResult<()> {
    let result = lib_it_views::render("views/default", &mut disguise::context!())?;
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
    let mut context = disguise::context! {
        name: "Disguise".to_string(),
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
        <div>Welcome! - from Disguise(1)</div>
    </body>
</html>"#
        .trim();
    assert_eq!(result.trim(), expected);

    Ok(())
}

#[test]
fn comp_home_view() -> result::RenderResult<()> {
    let result = lib_it_views::render("views/sub/home", &mut disguise::context!())?;
    assert!(result.contains("<title>Home</title>"));
    assert!(result.contains("<li>menu 1</li>"));
    assert!(result.contains("this is footer"));

    Ok(())
}
