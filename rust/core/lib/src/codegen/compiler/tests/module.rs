#![cfg(test)]
use std::path::PathBuf;

use crate::codegen::{CompilerOptions, compiler::module::Module};

#[test]
#[should_panic]
fn process_with_invalid_dir() {
    let dir = PathBuf::from("text.txt");
    let target_file = PathBuf::from("text.rs");
    let module = Module::new(dir, target_file, None);
    let compiler_options = CompilerOptions::default();
    module.process(&compiler_options).unwrap();
}
