#![cfg(test)]

use crate::types::functions;

#[test]
fn test_normalize_path_to_key() {
    assert_eq!(
        functions::normalize_path_to_view_key("test1/test2"),
        "test1::test2::Test2View"
    );

    assert_eq!(
        functions::normalize_path_to_view_key("test1/test2/"),
        "test1::test2::Test2View"
    );

    assert_eq!(
        functions::normalize_path_to_view_key("/test1/test2"),
        "test1::test2::Test2View"
    );

    assert_eq!(
        functions::normalize_path_to_view_key("/test1/test2/"),
        "test1::test2::Test2View"
    );
}

#[test]
fn test_resolve_layout_to_view_keys() {
    // absolute
    assert_eq!(
        functions::resolve_layout_to_view_keys("/layout", "test1::test2::Test2View"),
        vec![String::from("layout")]
    );

    // relative
    assert_eq!(
        functions::resolve_layout_to_view_keys("~/layout", "test1::test2::Test2View"),
        vec![String::from("test1/layout")]
    );

    // fallback
    assert_eq!(
        functions::resolve_layout_to_view_keys("p/layout", "test1::test2::Test2View"),
        vec![String::from("test1/p/layout"), String::from("p/layout")]
    );
}
