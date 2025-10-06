#![allow(dead_code)]
use crate::codegen::consts;
use crate::codegen::parser::tokenizer::{self, TokenStream};
use crate::codegen::types::Block;
use crate::types::{error, result};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::types) fn parse_render(
        source: &'a str,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        // token_stream starts with the directive.
        let start_token = match token_stream.peek_token() {
            Some(token) => match token.kind() {
                tokenizer::Kind::EXPRESSION
                    if consts::KEYWORD_RENDER_SECTION == &source[token.range()] =>
                {
                    token
                }
                _ => {
                    return Err(error::CompileError::from_parser(
                        source,
                        Some(*token),
                        &format!("Expected '{}' after '@'", consts::KEYWORD_RENDER_SECTION),
                    ));
                }
            },
            None => {
                return Err(error::CompileError::from_parser(
                    source,
                    None,
                    &format!("Expected '{}' after '@'", consts::KEYWORD_RENDER_SECTION),
                ));
            }
        };

        // consume the directive token
        token_stream.next_token();

        // todo: (), (exp, true|false)
        // parse_block_within => must return block with 2 childrend
        // none
        Err(error::CompileError::from_parser(
            source,
            Some(*start_token),
            "not implemented",
        ))
    }
}
