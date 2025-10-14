#![cfg(test)]
use crate::codegen::types::Block;
use crate::{
    codegen::types::Span,
    types::{
        Location,
        error::{self, CompileError},
    },
};
use proc_macro2::TokenStream;
use quote::format_ident;
use std::{io::ErrorKind, path::PathBuf};

#[test]
fn comiple_error_from_lex_error() {
    let lex_err = "(abc;".parse::<TokenStream>();
    assert!(lex_err.is_err());
    let lex_err = lex_err.err().unwrap();
    let block = Block::new_content(Span::new(""));
    let complie_err = CompileError::from_lex(&block, lex_err);
    assert!(matches!(complie_err, CompileError::CodeGen(_, _, _)));
}

#[test]
fn comiple_error_from_option_string() {
    let op_str = Some("test".to_string());
    let complie_err = CompileError::from(op_str);
    assert!(matches!(complie_err, CompileError::String(_)));

    let complie_err = CompileError::from(None);
    assert!(matches!(complie_err, CompileError::String(_)));
}

#[test]
fn comiple_error_from_std_error() {
    let std_error = std::io::Error::new(ErrorKind::Other, "error!");
    let complie_err = CompileError::from(std_error);
    assert!(matches!(complie_err, CompileError::String(_)));
}

#[test]
fn comiple_error_from_sync_error() {
    let name_token = format_ident!("test_ident");
    let syn_error = syn::Error::new(name_token.span(), "error!");
    let complie_err = CompileError::from(syn_error);
    assert!(matches!(complie_err, CompileError::String(_)));
}

#[test]
fn comiple_error_syn_error_with_file_info() {
    let name_token = format_ident!("test_ident");
    let syn_error = syn::Error::new(name_token.span(), "error!");
    let complie_err = CompileError::from(syn_error);

    let file = PathBuf::from("text.txt");
    let err_with_file = complie_err.with_file(&file);
    assert!(matches!(err_with_file, CompileError::FileError(_, _, _, _)));
}

#[test]
fn comiple_error_codegen_error_with_file_info() {
    let codegen_err =
        error::CompileError::CodeGen(Location::default(), "test".to_string(), "test".to_string());
    let file = PathBuf::from("text.txt");
    let err_with_file = codegen_err.with_file(&file);
    assert!(matches!(err_with_file, CompileError::FileError(_, _, _, _)));
}

#[test]
fn comiple_error_parser_error_with_file_info() {
    let codegen_err = error::CompileError::Parser(
        Some(Location::default()),
        "test".to_string(),
        "test".to_string(),
    );
    let file = PathBuf::from("text.txt");
    let err_with_file = codegen_err.with_file(&file);
    assert!(matches!(err_with_file, CompileError::FileError(_, _, _, _)));

    let codegen_err = error::CompileError::Parser(None, "test".to_string(), "".to_string());
    let err_with_file = codegen_err.with_file(&file);
    assert!(err_with_file.to_string().contains(file.to_str().unwrap()));
    assert!(matches!(err_with_file, CompileError::FileError(_, _, _, _)));
}

#[test]
fn comiple_error_file_error_with_file_info() {
    let codegen_err = error::CompileError::Parser(
        Some(Location::default()),
        "test".to_string(),
        "test".to_string(),
    );
    let file = PathBuf::from("text.txt");
    let err_with_file = codegen_err.with_file(&file);
    assert!(err_with_file.to_string().contains(file.to_str().unwrap()));
    assert!(matches!(err_with_file, CompileError::FileError(_, _, _, _)));
    let err_with_file2 = err_with_file.with_file(&file);
    assert!(matches!(
        err_with_file2,
        CompileError::FileError(_, _, _, _)
    ));
    assert_eq!(err_with_file, err_with_file2);
}

#[test]
fn comiple_error_file_error_without_detail_fmt() {
    let file = PathBuf::from("text.txt");
    let file_err =
        error::CompileError::FileError(file, Some(Location::default()), "test".to_string(), None);
    assert!(file_err.to_string().contains("text.txt"));
}
