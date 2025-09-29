#![cfg(test)]
use crate::codegen::parser::template::util;
use crate::{
    codegen::parser::{
        template::{Context, ParseContext},
        tokenizer::{self, Tokenizer},
    },
    types::error,
};
use winnow::stream::TokenSlice;

#[test]
fn get_token_before_transfer_end_with_eof() -> core::result::Result<(), error::Error> {
    let source = "test";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);

    let result = util::get_token_before_transfer(
        source,
        &mut token_stream,
        &ParseContext::new(Context::Content),
        |k| !vec![tokenizer::Kind::WHITESPACE, tokenizer::Kind::NEWLINE].contains(&k),
    );

    assert!(result.is_some());
    let token = result.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::EOF);
    Ok(())
}

#[test]
fn get_token_before_transfer_end_with_whitespace() -> core::result::Result<(), error::Error> {
    let source = "test ";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);

    let result = util::get_token_before_transfer(
        source,
        &mut token_stream,
        &ParseContext::new(Context::Content),
        |k| !vec![tokenizer::Kind::WHITESPACE].contains(&k),
    );

    assert!(result.is_some());
    let token = result.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::WHITESPACE);
    Ok(())
}

#[test]
fn get_token_before_transfer_end_with_ln() -> core::result::Result<(), error::Error> {
    let source = "test\n";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);

    let result = util::get_token_before_transfer(
        source,
        &mut token_stream,
        &ParseContext::new(Context::Content),
        |k| !vec![tokenizer::Kind::NEWLINE].contains(&k),
    );

    assert!(result.is_some());
    let token = result.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::NEWLINE);
    Ok(())
}

#[test]
fn get_token_before_transfer_end_with_transfer() -> core::result::Result<(), error::Error> {
    let source = "test@123";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);

    let result = util::get_token_before_transfer(
        source,
        &mut token_stream,
        &ParseContext::new(Context::Content),
        |k| !vec![tokenizer::Kind::NEWLINE, tokenizer::Kind::WHITESPACE].contains(&k),
    );

    assert!(result.is_some());
    let token = result.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::AT);
    Ok(())
}

#[test]
fn get_token_before_transfer_end_with_non_transfer() -> core::result::Result<(), error::Error> {
    let source = "test@@123";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);

    let result = util::get_token_before_transfer(
        source,
        &mut token_stream,
        &ParseContext::new(Context::Content),
        |k| !vec![tokenizer::Kind::NEWLINE].contains(&k),
    );

    assert!(result.is_some());
    let token = result.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::EOF);
    Ok(())
}
