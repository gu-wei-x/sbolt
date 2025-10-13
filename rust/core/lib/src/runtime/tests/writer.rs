#![cfg(test)]
use crate::types::{HtmlWriter, KWriter, Writer};

#[test]
fn html_writer() {
    let content = "hell world!";
    let mut html_writer = HtmlWriter::new();
    html_writer.write(content);
    html_writer.writeln("");
    html_writer.writefn(|| "test".to_string());
    assert_eq!(
        html_writer.into_string(),
        format!("{}\n{}", content, "test")
    );
}

#[test]
fn html_kwriter() {
    let content = "hell world!";
    let html_writer = HtmlWriter::new();
    let mut kwriter = KWriter::KHtml(html_writer);
    kwriter.write(content);
    assert_eq!(kwriter.into_string(), content)
}

#[test]
fn string_writer() {
    let content = "hell world!";
    let mut string_writer = String::new();
    string_writer.write(content);
    string_writer.writeln("");
    string_writer.writefn(|| "test".to_string());
    assert_eq!(
        string_writer.into_string(),
        format!("{}\n{}", content, "test")
    );
}

#[test]
fn string_kwriter() {
    let content = "hell world!";
    let string_writer = String::new();
    let mut kwriter = KWriter::KText(string_writer);
    kwriter.write(content);
    assert_eq!(kwriter.into_string(), content)
}

#[test]
fn json_kwriter() {
    let content = "hell world!";
    let string_writer = String::new();
    let mut kwriter = KWriter::KJson(string_writer);
    kwriter.write(content);
    assert_eq!(kwriter.into_string(), content)
}
