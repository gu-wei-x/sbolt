#![cfg(test)]
use winnow::stream::Stream;
use winnow::stream::TokenSlice;

use crate::codegen::parser::tokenizer;
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::parser::tokenizer::stream;

#[test]
fn skip_whitespace_false() {
    let source = "123";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let mut stream = TokenSlice::new(&tokens);
    let result = stream::skip_whitespace(&mut stream);
    assert!(!result)
}

#[test]
fn skip_whitespace_true() {
    let source = " 123";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let mut stream = TokenSlice::new(&tokens);
    let result = stream::skip_whitespace(&mut stream);
    assert!(result)
}

#[test]
fn skip_whitespace_and_newline_false() {
    let source = "\n123";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let mut stream = TokenSlice::new(&tokens);
    let result = stream::skip_whitespace_and_newline(&mut stream);
    assert!(result)
}

#[test]
fn get_next_token_if() {
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
fn get_next_token_util() {
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
fn skip_next_token_if() {
    let source = "\n123";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let mut stream = TokenSlice::new(&tokens);
    let result = stream::skip_next_token_if(&mut stream, |k| tokenizer::Kind::NEWLINE == k);
    assert!(result)
}

#[test]
fn get_nth_token() {
    let source = "\n123@*\n";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let stream = TokenSlice::new(&tokens);
    let first = stream::get_nth_token(&stream, 0);

    // first token is newline
    assert!(first.is_some());
    let first_token = first.unwrap();
    assert_eq!(first_token.kind(), tokenizer::Kind::NEWLINE);

    // second token is expression
    let second = stream::get_nth_token(&stream, 1);
    assert!(second.is_some());
    let token = second.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::EXPRESSION);

    // 3rd token is @
    let third = stream::get_nth_token(&stream, 2);
    assert!(third.is_some());
    let token = third.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::AT);

    // 4th token is *
    let fourth = stream::get_nth_token(&stream, 3);
    assert!(fourth.is_some());
    let token = fourth.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::ASTERISK);

    // 5th token is newline
    let fifth = stream::get_nth_token(&stream, 4);
    assert!(fifth.is_some());
    let token = fifth.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::NEWLINE);

    // 6th token is EOF
    let sixth = stream::get_nth_token(&stream, 5);
    assert!(sixth.is_some());
    let token = sixth.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::EOF);

    // unconsumed token
    let unconsumed = stream.peek_token();
    assert!(unconsumed.is_some());
    let token = unconsumed.unwrap();
    assert_eq!(token.kind(), tokenizer::Kind::NEWLINE);
    assert_eq!(token, first_token);
}
