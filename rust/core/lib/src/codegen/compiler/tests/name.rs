#![cfg(test)]
use crate::codegen::compiler::name;

#[test]
fn create_name_space() {
    let ns_prefix = None;
    let name = String::from("test");
    let full_name = name::create_name_space(&ns_prefix, &name);
    assert_eq!(full_name, name);

    let ns_prefix = Some(String::from("n1"));
    let name = String::from("test");
    let full_name = name::create_name_space(&ns_prefix, &name);
    assert_eq!(full_name, String::from("n1::test"));

    let ns_prefix = Some(String::from("n1::n2"));
    let name = String::from("test");
    let full_name = name::create_name_space(&ns_prefix, &name);
    assert_eq!(full_name, String::from("n1::n2::test"));
}

#[test]
fn create_view_type_name() {
    assert_eq!(name::create_view_type_name("index"), "IndexView");
}

#[test]
fn create_type_full_name() {
    assert_eq!(
        name::create_type_full_name("index", "test"),
        "crate::test::index"
    );
}
