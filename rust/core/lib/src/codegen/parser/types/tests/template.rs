#![cfg(test)]
use crate::codegen::types::Template;

#[test]
#[should_panic]
fn template_from_empty() {
    Template::from("", None).unwrap();
}
