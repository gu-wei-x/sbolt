#![cfg(test)]
use crate::codegen::consts;
use crate::codegen::parser::template::Block;
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::types::error;
use winnow::stream::{Stream, TokenSlice};

#[test]
fn test_parse_directive_use() -> core::result::Result<(), error::Error> {
    let use_statement = "use std::fmt";
    let use_contents = [
        use_statement,
        &format!("{};", use_statement),
        &format!("{}\n", use_statement),
    ];

    for use_content in use_contents {
        let content = &format!("@{}", use_content);
        let tokenizer = Tokenizer::new(content);
        let tokens = tokenizer.into_vec();
        let mut token_stream = TokenSlice::new(&tokens);
        let start_token = token_stream
            .peek_token()
            .ok_or_else(|| error::Error::from_parser(None, "Expected '@'"))?;
        let block = Block::parse_code(content, start_token, &mut token_stream)?;
        assert_eq!(block.name, Some(consts::KEYWORD_USE.to_string()));
        assert_eq!(block.content(), use_statement);
    }

    Ok(())
}

#[test]
fn test_parse_directive_use_illegal() {
    let use_statement = "use";
    let use_contents = [
        use_statement,
        &format!("{} ", use_statement),
        &format!("{};", use_statement),
        &format!("{}\n", use_statement),
        &format!("{} ;", use_statement),
        &format!("{} \n", use_statement),
    ];

    for use_content in use_contents {
        let content = &format!("@{}", use_content);
        let tokenizer = Tokenizer::new(content);
        let tokens = tokenizer.into_vec();
        let mut token_stream = TokenSlice::new(&tokens);
        let start_token = token_stream.peek_token().unwrap();
        let resut = Block::parse_code(content, start_token, &mut token_stream);
        assert!(resut.is_err());
    }
}

#[test]
fn test_parse_directive_layout() {
    panic!("TODO: impl")
}

#[test]
fn test_parse_directive_layout_illegal() {
    panic!("TODO: impl")
}
