#![cfg(test)]
use crate::codegen::parser::html::parse_html;
use crate::types::result;

#[test]
fn parse_wellformed_node_with_no_meaningful_content() -> result::Result<()> {
    let source = "<div>\n  </div>";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    let expected = "<div/>";
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn parse_wellformed_node_with_attrs() -> result::Result<()> {
    let source = r#"
        <div a1="a1" a2="a2">
           test
        </div>
    "#;
    let dom = parse_html(source)?;
    let content = dom.to_string();
    let expected = r#"<div a1="a1" a2="a2">test</div>"#;
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn parse_wellformed_node_with_spaces_in_attrs() -> result::Result<()> {
    // note: not supported for multiple spaces between atrrite parts.
    let source = r#"
        <div a=" a0 a1 a2 a3 " b="b0 b1 b2 b3 ">
           test
        </div>
    "#;
    let dom = parse_html(source)?;
    let content = dom.to_string();
    let expected = r#"<div a="a0 a1 a2 a3" b="b0 b1 b2 b3">test</div>"#;
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn parse_open_tag() -> result::Result<()> {
    let source = r#"
        <div a1="a1" a2="a2"
    "#;
    let dom = parse_html(source)?;
    let content = dom.to_string();
    let expected = r#"<div a1="a1" a2="a2""#;
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn parse_fragments() -> result::Result<()> {
    let source = r#"
        </div>
        <div a1="a1" a2="a2">
           test
        </div>
    "#;
    let dom = parse_html(source)?;
    let content = dom.to_string();
    let expected = r#"</div><div a1="a1" a2="a2">test</div>"#;
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn parse_fragments2() -> result::Result<()> {
    let source = r#"
        <div>Welcome: "#;
    let dom = parse_html(source)?;
    let content = dom.to_string();
    let expected = r#"<div>Welcome: "#;
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn parse_node_with_whitespaces() -> result::Result<()> {
    let source = r#"
        < div a1="a0"    a2="a2" >
           test
        < / div >
    "#;
    let dom = parse_html(source)?;
    let content = dom.to_string();
    let expected = r#"<div a1="a0" a2="a2">test</div>"#;
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn parse_nested_nodes() -> result::Result<()> {
    let source = r#"
        <div ap1="ap1">
           test
           <div ap2="ap2">
              test2
           </div>
        </div>
    "#;
    let dom = parse_html(source)?;
    let content = dom.to_string();
    let expected = "<div ap1=\"ap1\">test<div ap2=\"ap2\">test2</div></div>";
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn parse_fragments_complicated() -> result::Result<()> {
    let source = r#"
        <html>
            <head>
               <title>Welcome</title>
            </head>
        <body>"#;
    let dom = parse_html(source)?;
    let content = dom.to_string();
    let expected = "<html><head><title>Welcome</title></head><body>";
    assert_eq!(content, expected);

    Ok(())
}

// <div>@msg - from @name(@age)</div>

#[test]
fn parse_fragments_text() -> result::Result<()> {
    let source = " - from ";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    assert_eq!(content, source);

    Ok(())
}

#[test]
fn parse_doc_type_html5_node() -> result::Result<()> {
    // html5: <!DOCTYPE html>
    let source = "<!doctype html>";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    assert_eq!(content, source);

    Ok(())
}

#[test]
fn parse_doc_type_html4_node() -> result::Result<()> {
    // html 4.01: <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
    let source = "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\" \"http://www.w3.org/TR/html4/loose.dtd\">";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    assert_eq!(content, source);

    Ok(())
}

#[test]
fn parse_doc_type_xhtml_node() -> result::Result<()> {
    // XHTML 1.1: <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
    let source = "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\" \"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    assert_eq!(content, source);

    Ok(())
}

#[test]
fn parse_comment_type_node() -> result::Result<()> {
    let source = r#"
       <!-- test -->
    "#;
    let dom = parse_html(source)?;
    let content = dom.to_string();
    assert_eq!(content, "");

    Ok(())
}

#[test]
fn parse_html_open_tag() -> result::Result<()> {
    let source = "  <title>";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    assert_eq!(content, "<title>");

    Ok(())
}

#[test]
fn parse_void_tag() -> result::Result<()> {
    // In HTML5, closing tags for tags like link , meta , img , hr , br are not mandatory.
    // But if following XHTML principles, it is considered to add closing tags to those HTML tags
    // (ex: "<meta />, <link />, <img />, <hr/>, <br/>")
    let source = "<head><link rel=\"dns-prefetch\" href=\"https://www.test.com\">test<br></head>";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    // here: close the tag to follow XHTML principles.
    let expected =
        "<head><link rel=\"dns-prefetch\" href=\"https://www.test.com\"/>test<br/></head>";
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn parse_html_custom_tagname() -> result::Result<()> {
    // Custom Element
    let source = "<test-tag name=\"route-action\" content=\"pull_request_layout\">";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    assert_eq!(content, source);

    let source = "<test_tag name=\"route-action\" content=\"pull_request_layout\">";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    assert_eq!(content, source);

    let source = "<test:tag name=\"route-action\" content=\"pull_request_layout\">";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    assert_eq!(content, source);

    Ok(())
}

#[test]
fn parse_html_custome_attr_name() -> result::Result<()> {
    let source =
        "<meta name=\"route-action\" content=\"pull_request_layout\" data-turbo-transient>";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    let expected =
        "<meta name=\"route-action\" content=\"pull_request_layout\" data-turbo-transient/>";
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn parse_html_attr_without_value() -> result::Result<()> {
    let source =
        "<meta name=\"route-action\" content=\"pull_request_layout\" data-turbo-transient>";
    let dom = parse_html(source)?;
    let content = dom.to_string();

    // close meta to following xhtml principles.
    let expected =
        "<meta name=\"route-action\" content=\"pull_request_layout\" data-turbo-transient/>";
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn parse_html_void_tag_with_close_tag() -> result::Result<()> {
    let source = "<link rel=\"dns-prefetch\" href=\"https://www.test.com\"></link>";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    // extra </link> is ilegal but leave it to browser.
    let expected = "<link rel=\"dns-prefetch\" href=\"https://www.test.com\"/></link>";
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn parse_html_self_close_tag_without_attribute() -> result::Result<()> {
    let source = "<test/>";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    let expected = "<test/>";
    assert_eq!(content, expected);

    let source = "<test></test>";
    let dom = parse_html(source)?;
    let content = dom.to_string();
    let expected = "<test/>";
    assert_eq!(content, expected);

    Ok(())
}
