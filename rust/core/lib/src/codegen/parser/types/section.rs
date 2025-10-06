#![allow(dead_code)]
use crate::codegen::consts;
use crate::codegen::parser::Token;
use crate::codegen::parser::tokenizer::{self, TokenStream};
use crate::codegen::parser::types::context::{Kind, ParseContext};
use crate::codegen::types::Block;
use crate::types::{error, result};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    pub(in crate::codegen) fn parse_section(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        // consume the section token
        token_stream.next_token();

        // whitespace after layout token
        if !tokenizer::skip_whitespace(token_stream) {
            return Err(error::CompileError::from_parser(
                source,
                Some(*token),
                &format!("Expected whitespace after '@{}'", consts::KEYWORD_SECTION),
            ));
        }

        match token_stream.peek_token() {
            None => Err(error::CompileError::from_parser(
                source,
                Some(*token),
                &format!(
                    "Expected {} name after '@{}'",
                    consts::KEYWORD_SECTION,
                    consts::KEYWORD_SECTION
                ),
            )),
            Some(start_token) => match start_token.kind() {
                tokenizer::Kind::EXPRESSION => {
                    let name = &source[start_token.range()];
                    // consume the expression token.
                    token_stream.next_token();

                    // whitespace after section name
                    tokenizer::skip_whitespace(token_stream);
                    match token_stream.peek_token() {
                        Some(brace_token)
                            if brace_token.kind() == tokenizer::Kind::OCURLYBRACKET =>
                        {
                            let mut context = ParseContext::new(Kind::KSECTION);
                            let mut block = Self::parse_block_within_kinds(
                                source,
                                tokenizer::Kind::OCURLYBRACKET,
                                tokenizer::Kind::CCURLYBRACKET,
                                token_stream,
                                &mut context,
                            )?;
                            block.with_name(name);
                            Ok(block)
                        }
                        _ => Err(error::CompileError::from_parser(
                            source,
                            Some(*token),
                            &format!(
                                "Expected '{{' after '@{} {}'",
                                consts::KEYWORD_SECTION,
                                name
                            ),
                        )),
                    }
                }
                _ => Err(error::CompileError::from_parser(
                    source,
                    Some(*token),
                    &format!(
                        "Expected {} name after '@{}'",
                        consts::KEYWORD_SECTION,
                        consts::KEYWORD_SECTION
                    ),
                )),
            },
        }
    }
}
