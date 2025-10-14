#![cfg(test)]
use crate::codegen::CompileResult;

#[test]
fn compile_result() {
    let mut result = CompileResult::new();
    result.add_warning("test".into());
    let warnings = result.warnings();
    assert!(!warnings.is_empty())
}
