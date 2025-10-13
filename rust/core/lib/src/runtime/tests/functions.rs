#![cfg(test)]
use crate::types;

#[test]
fn normalize_path_to_key() {
    assert_eq!(
        types::normalize_path_to_view_key("test1/test2"),
        Some(String::from("test1::test2::Test2View"))
    );

    assert_eq!(
        types::normalize_path_to_view_key("test1/test2/"),
        Some(String::from("test1::test2::Test2View"))
    );

    assert_eq!(
        types::normalize_path_to_view_key("/test1/test2"),
        Some(String::from("test1::test2::Test2View"))
    );

    assert_eq!(
        types::normalize_path_to_view_key("/test1/test2/"),
        Some(String::from("test1::test2::Test2View"))
    );

    assert_eq!(types::normalize_path_to_view_key(""), None);
}

#[test]
fn resolve_layout_to_view_keys() {
    // absolute
    assert_eq!(
        types::resolve_layout_to_view_keys("/layout", "test1::test2::Test2View"),
        vec![String::from("layout")]
    );

    // relative
    assert_eq!(
        types::resolve_layout_to_view_keys("~/layout", "test1::test2::Test2View"),
        vec![String::from("test1/layout")]
    );

    // fallback
    assert_eq!(
        types::resolve_layout_to_view_keys("p/layout", "test1::test2::Test2View"),
        vec![String::from("test1/p/layout"), String::from("p/layout")]
    );
}
