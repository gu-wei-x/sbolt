use crate::codegen::CompileResult;

#[test]
fn test_compile_result() {
    let result = CompileResult::new();
    assert!(result.is_success());
}
