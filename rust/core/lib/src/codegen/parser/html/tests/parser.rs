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
