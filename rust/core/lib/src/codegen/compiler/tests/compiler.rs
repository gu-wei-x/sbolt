#![cfg(test)]
use crate::codegen;

#[test]
#[should_panic]
fn compile_with_invalid_name() {
    let option = codegen::CompilerOptions::default().with_mod_name("test-mod_name");
    let compiler = codegen::Compiler::new(option);
    compiler.compile();
}

#[test]
fn compiler_options() {
    let option = codegen::CompilerOptions::default()
        .with_out_dir("src")
        .with_debug(false);
    assert_eq!(&option.out_dir, &Some("src".to_string()));
    assert!(!option.debug)
}
