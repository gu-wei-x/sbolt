#![cfg(test)]
use sbolt::types::result;
sbolt::include_views!();

#[test]
fn optimized_html_preserves_structure_and_attribute_values() -> result::RenderResult<()> {
    let result = lib_it_op_views::render("views/html_regressions", &mut sbolt::context!())?;
    assert_eq!(
        result.trim(),
        r#"<div></div><my-widget>hello</my-widget><p title='say "hi" &amp; goodbye'>after</p>"#
    );
    Ok(())
}

#[test]
fn static_compaction_respects_sections_and_loops() -> result::RenderResult<()> {
    let result = lib_it_op_views::render("views/boundaries", &mut sbolt::context!())?;
    assert!(
        result.starts_with("<div class='x'>ab</div><p>block</p><span>one</span> <span>two</span>")
    );
    assert!(result.contains("<pre><span   title = ' a  b '> hello  world </span></pre>"));
    assert!(result.contains("<span   data-x = 'y'>0 </span><span   data-x = 'y'>1 </span>"));
    Ok(())
}

#[test]
fn default_view() -> result::RenderResult<()> {
    let result = lib_it_op_views::render("views/default", &mut sbolt::context!())?;
    let expected = r#"<html><head>
        <title>Default</title>
    </head>
    <body>
        <div>Hello Default!</div>
    </body>
</html>"#;
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
    let result = lib_it_op_views::render("views/sub/index", &mut context)?;
    let expected = "<html>\n    <head>\n        <title>Welcome</title>\n    </head>\n    <body>\n        <div>Welcome! - from sbolt(1)</div>\n    </body>\n</html>";
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn sub_home_view() -> result::RenderResult<()> {
    let result = lib_it_op_views::render("views/sub/home", &mut sbolt::context!())?;
    let expected = "<html><head>\n        \n    <title>Home</title>\n    </head>\n    <body>\n        <div class=\"menu\">\n           \n    <ul>\n        <li>menu 1</li>\n        <li>menu 2</li>\n        <li>menu 3</li>\n    </ul>\n        </div>\n        <div class=\"main\">\n           <div>Hello world!<div>        </div>\n        <div class=\"footer\">\n           \n    this is footer\n        </div>\n    </body>\n</html>";
    assert_eq!(result, expected);

    Ok(())
}
