#![cfg(test)]
use winnow::stream::TokenSlice;

use crate::codegen::parser::tokenizer;
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::parser::tokenizer::stream;

#[test]
fn test_skip_whitespace_false() {
    let source = "123";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let mut stream = TokenSlice::new(&tokens);
    let result = stream::skip_whitespace(&mut stream);
    assert!(!result)
}

#[test]
fn test_skip_whitespace_true() {
    let source = " 123";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let mut stream = TokenSlice::new(&tokens);
    let result = stream::skip_whitespace(&mut stream);
    assert!(result)
}

#[test]
fn test_skip_whitespace_and_newline_false() {
    let source = "\n123";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let mut stream = TokenSlice::new(&tokens);
    let result = stream::skip_whitespace_and_newline(&mut stream);
    assert!(result)
}

#[test]
fn test_get_next_token_if() {
    let source = "\n123";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let mut stream = TokenSlice::new(&tokens);
    let result = stream::get_next_token_if(&mut stream, |k| tokenizer::Kind::NEWLINE == k);
    assert!(result.is_some());
    let token = result.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::EXPRESSION);
}

#[test]
fn test_get_next_token_util() {
    let source = "\n123";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let mut stream = TokenSlice::new(&tokens);
    let result = stream::get_next_token_util(&mut stream, |k| tokenizer::Kind::NEWLINE == k);
    assert!(result.is_some());
    let token = result.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::NEWLINE);
}

#[test]
fn test_skip_next_token_if() {
    let source = "\n123";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let mut stream = TokenSlice::new(&tokens);
    let result = stream::skip_next_token_if(&mut stream, |k| tokenizer::Kind::NEWLINE == k);
    assert!(result)
}
