#![cfg(test)]
use crate::codegen;

#[test]
#[should_panic]
fn compile_with_invalid_name() {
    let option = codegen::CompilerOptions::default().with_mod_name("test-mod_name");
    let compiler = codegen::Compiler::new(option);
    compiler.compile();
}
