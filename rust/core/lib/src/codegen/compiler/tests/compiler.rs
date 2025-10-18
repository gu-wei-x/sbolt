#![cfg(test)]
use crate::{
    codegen::{self, consts},
    types::template,
};

#[test]
#[should_panic]
fn compile_with_invalid_name() {
    let option = codegen::CompilerOptions::default().with_mod_name("test-mod_name");
    let compiler = codegen::Compiler::new(option);
    compiler.compile();
}

#[test]
fn compiler_options_default() {
    let option = codegen::CompilerOptions::default();
    let default_extensions = option.extensions();
    assert_eq!(default_extensions.len(), 3);
    assert_eq!(
        default_extensions.get(consts::DEFAULT_HTML_TEMPLATE_FILE_EXTENSION),
        Some(&template::Kind::KHTML)
    );
    assert_eq!(
        default_extensions.get(consts::DEFAULT_JSON_TEMPLATE_FILE_EXTENSION),
        Some(&template::Kind::KJSON)
    );
    assert_eq!(
        default_extensions.get(consts::DEFAULT_TEXT_TEMPLATE_FILE_EXTENSION),
        Some(&template::Kind::KTEXT)
    );
    assert!(!option.mod_name().is_empty());
    #[cfg(debug_assertions)]
    assert!(!option.need_optimization());
    #[cfg(not(debug_assertions))]
    assert!(option.need_optimization());
    assert!(option.out_dir().is_none());
    assert!(option.source_dirs().is_empty());
}

#[test]
fn compiler_options_set() {
    let option = codegen::CompilerOptions::default()
        .with_extension("rshtm", template::Kind::KHTML)
        .with_mod_name("test_views")
        .with_optimization(true)
        .with_out_dir("temp")
        .with_source_dir("views");

    let extensions = option.extensions();
    assert_eq!(extensions.get("rshtm"), Some(&template::Kind::KHTML));
    assert_eq!(option.mod_name(), "test_views");
    assert!(option.need_optimization());
    assert_eq!(option.out_dir(), &Some(String::from("temp")));
    assert_eq!(option.source_dirs(), &["views"]);
}
