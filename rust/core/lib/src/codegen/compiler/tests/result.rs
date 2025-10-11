#![cfg(test)]
use crate::codegen::CompileResult;

#[test]
fn compile_result() {
    let result = CompileResult::new();
    assert!(result.is_success());
}
