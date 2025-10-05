#![allow(dead_code)]
use crate::codegen::consts;
use crate::codegen::parser::Token;
use crate::codegen::parser::tokenizer::{self, TokenStream, get_nth_token};
use crate::codegen::types::Block;
use crate::codegen::types::Span;
use crate::types::{error, result};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::types) fn parse_directive(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
        directive: &str,
    ) -> result::Result<Block<'a>> {
        // consume the directive token
        token_stream.next_token();

        // whitespace after directive token
        if !tokenizer::skip_whitespace(token_stream) {
            return Err(error::CompileError::from_parser(
                source,
                Some(*token),
                &format!("Expected whitespace name after '@{directive}'"),
            ));
        }

        // validate directive content.
        let next_token = get_nth_token(token_stream, 0);
        if None == next_token {
            return Err(error::CompileError::from_parser(
                source,
                Some(*token),
                &format!("Expected {directive} content after '@{directive}'"),
            ));
        }

        let mut span = Span::new(source);
        while let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::NEWLINE | tokenizer::Kind::SEMICOLON => {
                    // consume the end token.
                    token_stream.next_token();
                    break;
                }
                _ => {
                    span.push_token(*token);
                    token_stream.next_token();
                }
            }
        }

        // TODO: check content
        /*let content = result.content();
        if content.trim().is_empty() {
            return Err(error::CompileError::from_parser(
                source,
                Some(*token),
                &format!("Expected {directive} content after '@{directive}'"),
            ));
        }*/

        match directive {
            consts::DIRECTIVE_KEYWORD_LAYOUT => Ok(Block::new_layout(span)),
            consts::DIRECTIVE_KEYWORD_USE => Ok(Block::KUSE(span)),
            _ => Err(error::CompileError::from_parser(
                source,
                Some(*token),
                &format!("Unknown {directive} after '@{directive}'"),
            )),
        }
    }
}
