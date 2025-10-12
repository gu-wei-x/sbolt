#![cfg(test)]
use crate::types::error::RuntimeError;

#[test]
fn view_not_found() {
    let error = RuntimeError::view_not_found("test");
    let err_msg = error.to_string();
    assert_eq!(err_msg, "View:test, NotFound: View 'test' not found");
}

#[test]
fn layout_not_found() {
    let error = RuntimeError::layout_not_found("layout", "test");
    let err_msg = error.to_string();
    assert_eq!(
        err_msg,
        "View:test, NotFound: Layout 'layout' not found for View `test`"
    );
}
