#![cfg(test)]
use crate::codegen::parser::html::parse_html;

#[test]
fn parse_wellformed_node_with_no_meaningful_content() {
    let source = "<div>\n  </div>";
    let dom = parse_html(source);
    let content = dom.to_string();
    let expected = "<div/>";
    assert_eq!(content, expected);
}

#[test]
fn parse_wellformed_node_with_attrs() {
    let source = r#"
        <div a1="a1" a2="a2">
           test
        </div>
    "#;
    let dom = parse_html(source);
    let content = dom.to_string();
    let expected = r#"<div a1="a1" a2="a2">test</div>"#;
    assert_eq!(content, expected);
}

#[test]
fn parse_wellformed_node_with_spaces_in_attrs() {
    // note: not supported for multiple spaces between atrrite parts.
    let source = r#"
        <div a=" a0 a1 a2 a3 " b="b0 b1 b2 b3 ">
           test
        </div>
    "#;
    let dom = parse_html(source);
    let content = dom.to_string();
    let expected = r#"<div a="a0 a1 a2 a3" b="b0 b1 b2 b3">test</div>"#;
    assert_eq!(content, expected);
}

#[test]
fn parse_fragments() {
    let source = r#"
        </div>
        <div a1="a1" a2="a2">
           test
        </div>
    "#;
    let dom = parse_html(source);
    let content = dom.to_string();
    let expected = r#"</div><div a1="a1" a2="a2">test</div>"#;
    assert_eq!(content, expected);
}

#[test]
fn parse_node_with_whitespaces() {
    let source = r#"
        < div a1="a0"    a2="a2" >
           test
        < / div >
    "#;
    let dom = parse_html(source);
    let content = dom.to_string();
    let expected = r#"<div a1="a0" a2="a2">test</div>"#;
    assert_eq!(content, expected);
}

#[test]
fn parse_nested_nodes() {
    let source = r#"
        <div ap1="ap1">
           test
           <div ap2="ap2">
              test2
           </div>
        </div>
    "#;
    let dom = parse_html(source);
    let content = dom.to_string();
    let expected = "<div ap1=\"ap1\">test<div ap2=\"ap2\">test2</div></div>";
    assert_eq!(content, expected);
}

#[test]
fn parse_fragments_complicated() {
    let source = r#"
        <html>
            <head>
               <title>Welcome</title>
            </head>
        <body>"#;
    let dom = parse_html(source);
    let content = dom.to_string();
    let expected = "<html><head><title>Welcome</title></head><body>";
    assert_eq!(content, expected);
}

// <div>@msg - from @name(@age)</div>

#[test]
fn parse_fragments_text() {
    let source = " - from ";
    let dom = parse_html(source);
    let content = dom.to_string();
    assert_eq!(content, source);
}

#[test]
fn parse_doc_type_html5_node() {
    // html5: <!DOCTYPE html>
    let source = "<!doctype html>";
    let dom = parse_html(source);
    let content = dom.to_string();
    assert_eq!(content, source);
}

#[test]
fn parse_doc_type_html4_node() {
    // html 4.01: <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
    let source = "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\" \"http://www.w3.org/TR/html4/loose.dtd\">";
    let dom = parse_html(source);
    let content = dom.to_string();
    assert_eq!(content, source);
}

#[test]
fn parse_doc_type_xhtml_node() {
    // XHTML 1.1: <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">
    let source = "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\" \"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">";
    let dom = parse_html(source);
    let content = dom.to_string();
    assert_eq!(content, source);
}

#[test]
fn parse_comment_type_node() {
    let source = r#"
       <!-- test -->
    "#;
    let dom = parse_html(source);
    let content = dom.to_string();
    assert_eq!(content, "");
}

#[test]
fn parse_html_open_tag() {
    let source = "  <title>";
    let dom = parse_html(source);
    let content = dom.to_string();
    assert_eq!(content, "<title>");
}

#[test]
fn parse_html_self_close_tag() {
    // In HTML5, closing tags for tags like link , meta , img , hr , br are not mandatory.
    // But if following XHTML principles, it is considered to add closing tags to those HTML tags
    // (ex: "<meta />, <link />, <img />, <hr/>, <br/>")
    let source = "<head><link rel=\"dns-prefetch\" href=\"https://www.test.com\">test<br></head>";
    let dom = parse_html(source);
    let content = dom.to_string();
    // here: close the tag to follow XHTML principles.
    let expected =
        "<head><link rel=\"dns-prefetch\" href=\"https://www.test.com\"/>test<br/></head>";
    assert_eq!(content, expected);
}

#[test]
fn parse_html_custom_tagname() {
    // Custom Element
    let source = "<test-tag name=\"route-action\" content=\"pull_request_layout\">";
    let dom = parse_html(source);
    let content = dom.to_string();
    assert_eq!(content, source);

    let source = "<test_tag name=\"route-action\" content=\"pull_request_layout\">";
    let dom = parse_html(source);
    let content = dom.to_string();
    assert_eq!(content, source);

    let source = "<test:tag name=\"route-action\" content=\"pull_request_layout\">";
    let dom = parse_html(source);
    let content = dom.to_string();
    assert_eq!(content, source);
}

#[test]
fn parse_html_custome_attr_name() {
    let source =
        "<meta name=\"route-action\" content=\"pull_request_layout\" data-turbo-transient>";
    let dom = parse_html(source);
    let content = dom.to_string();
    assert_eq!(content, source);
}

#[test]
fn parse_html_attr_without_value() {
    let source =
        "<meta name=\"route-action\" content=\"pull_request_layout\" data-turbo-transient>";
    let dom = parse_html(source);
    let content = dom.to_string();

    // close meta to following xhtml principles.
    let expected =
        "<meta name=\"route-action\" content=\"pull_request_layout\" data-turbo-transient/>";
    assert_eq!(content, expected);
}
