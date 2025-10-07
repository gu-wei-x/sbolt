#![cfg(test)]
// cargo test --test view_tests -- --nocapture
disguise::include_views!();

#[test]
fn comp_test_view() {
    let result = basic_views::render("views/test", disguise::context!());
    assert!(result.is_ok());
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
    let output = result.unwrap();
    println!("{output}");
    assert_eq!(output.trim(), expected);
}

#[test]
fn comp_index_view() {
    let context = disguise::context! {
        name: "Disguise".to_string(),
        age: 1,
        msg: "Hello world!".to_string()
    };
    let result = basic_views::render("views/comp/index", context);
    assert!(result.is_ok());
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
    let output = result.unwrap();
    assert_eq!(output.trim(), expected);
}

#[test]
fn comp_home_view() {
    let result = basic_views::render("views/comp/home", disguise::context!());
    assert!(result.is_ok());
}
