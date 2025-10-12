#![cfg(test)]
use crate::types::{HtmlWriter, Writer};

#[test]
fn html_writer() {
    let content = "hell world!";
    let mut html_writer = HtmlWriter::new();
    html_writer.write(content);
    assert_eq!(html_writer.into_string(), content)
}

#[test]
fn string_writer() {
    let content = "hell world!";
    let mut string_writer = String::new();
    string_writer.write(content);
    assert_eq!(string_writer.into_string(), content)
}
