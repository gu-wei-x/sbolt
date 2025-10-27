#![cfg(test)]
use crate::{codegen::parser::json::parse_json, types::result};

#[test]
fn parse_empty_string() -> result::Result<()> {
    let source = "";
    let object = parse_json(source)?;
    println!("{object:#?}");
    Ok(())
}
