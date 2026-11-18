#![cfg(test)]
use crate::{codegen::parser::json::parse_json, types::result};

#[test]
fn parses_and_compacts_nested_json_without_rewriting_values() -> result::Result<()> {
    let source = r#" {
        "z" : [ true, false, null, {}, [], -0, 1.2300e+999 ],
        "a" : { "text" : " a  b \n \t \" \\ \/ \b \f \r \u0041 \uD834\uDD1E " },
        "z" : 9007199254740993123456789
    } "#;
    let expected = r#"{"z":[true,false,null,{},[],-0,1.2300e+999],"a":{"text":" a  b \n \t \" \\ \/ \b \f \r \u0041 \uD834\uDD1E "},"z":9007199254740993123456789}"#;
    assert_eq!(parse_json(source)?.to_compact_string(), expected);
    assert_eq!(parse_json(expected)?.to_compact_string(), expected);
    Ok(())
}

#[test]
fn accepts_all_root_value_kinds_and_json_whitespace() -> result::Result<()> {
    for source in [
        "{}",
        "[]",
        "\"text\"",
        "0",
        "-12.3E-4",
        "true",
        "false",
        "null",
        r#""\uD800""#,
        r#""\uDC00""#,
    ] {
        assert_eq!(
            parse_json(&format!("\t\r\n {source} \n\t"))?.to_compact_string(),
            source
        );
    }
    let source = "{\"\\u0061\":\"\u{e9}\u{1f600}\",\"\u{e9}\":[1,2]}";
    assert_eq!(parse_json(source)?.to_compact_string(), source);
    Ok(())
}

#[test]
fn rejects_malformed_or_incomplete_json() {
    for source in [
        "",
        " \r\n\t",
        "{",
        "[",
        "}",
        "]",
        "{]",
        "[}",
        "[1,]",
        "{\"x\":1,}",
        "{\"x\" 1}",
        "{\"x\":}",
        "{true:1}",
        "{'x':1}",
        "{\"x\":1 \"y\":2}",
        "[1 2]",
        "true false",
        "{}[]",
        "truex",
        "tru",
        "False",
        "undefined",
        "NaN",
        "Infinity",
        "-",
        "+1",
        "01",
        "-01",
        ".1",
        "1.",
        "1e",
        "1e+",
        "--1",
        "1e2.3",
        "\"",
        "\"\\",
        "\"\\q\"",
        "\"\\u123\"",
        "\"\\u12xz\"",
        "\"a\nb\"",
        "\"a\tb\"",
        "\"\0\"",
        "\u{a0}null",
        "null\u{000b}",
        "/* comment */{}",
        "[// comment\n1]",
    ] {
        assert!(parse_json(source).is_err(), "Accepted {source:?}");
    }
}

#[test]
fn reports_errors_at_utf8_safe_locations() {
    let source = "{\n  \"x\": \"\u{e9}\",\n  \"y\": \u{1f600}\n}";
    let error = parse_json(source).unwrap_err();
    match error {
        crate::types::error::CompileError::Parser(Some(location), message, detail) => {
            assert_eq!(location.line, 2);
            assert_eq!(location.column, 7);
            assert_eq!(message, "Expected a JSON value");
            assert!(detail.starts_with('\u{1f600}'));
        }
        error => panic!("Expected a located parser error, got {error:?}"),
    }
}

#[test]
fn limits_nesting_without_panicking() -> result::Result<()> {
    let source = format!("{}0{}", "[".repeat(128), "]".repeat(128));
    assert_eq!(parse_json(&source)?.to_compact_string(), source);
    let source = format!("{}0{}", "[".repeat(129), "]".repeat(129));
    assert!(parse_json(&source).is_err());
    let source = format!("{}0{}", "{\"x\":".repeat(129), "}".repeat(129));
    assert!(parse_json(&source).is_err());
    Ok(())
}
