#![cfg(test)]
use disguise::types::result;
disguise::include_views!();

#[test]
#[should_panic]
fn no_existing_view() {
    basic_views::render("views/no_existing", disguise::context!()).unwrap();
}

#[test]
fn comp_test_view() -> result::RenderResult<()> {
    let result = basic_views::render("views/test", disguise::context!())?;
    let expected = r#"
<html>
    <head>
        <title>Test</title>
    </head>
    <body>
        <div>Hello Test!</div>
    </body>
</html>"#
        .trim();
    assert_eq!(result.trim(), expected);

    Ok(())
}

#[test]
fn comp_index_view() -> result::RenderResult<()> {
    let context = disguise::context! {
        name: "Disguise".to_string(),
        age: 1,
        msg: "Hello world!".to_string()
    };
    let result = basic_views::render("views/comp/index", context)?;
    let expected = r#"
<html>
    <head>
        <title>Index</title>
    </head>
    <body>
        <div>Hello world! - from Disguise(1)</div>
    </body>
</html>"#
        .trim();
    assert_eq!(result.trim(), expected);

    Ok(())
}

#[test]
fn comp_home_view() -> result::RenderResult<()> {
    let result = basic_views::render("views/comp/home", disguise::context!())?;
    println!("********************************");
    println!("{result:#?}");
    println!("********************************");

    Ok(())
}
