#![cfg(test)]
use crate::codegen::parser::html::doc;

#[test]
fn parse_string() {
    let source = r#"<div>\n  </div>"#;
    let html_doc = doc::HtmlDocument::parse(source);
    println!("{html_doc:#?}");
}
