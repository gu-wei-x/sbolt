#![allow(dead_code)]
use crate::codegen::consts;
use crate::codegen::parser::types::context::ParseContext;
use crate::codegen::parser::{Token, tokenizer};
use crate::codegen::types::Span;
use crate::codegen::{parser::tokenizer::TokenStream, types::Block};
use crate::types::{error, result};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    // @exp, @{}, @()
    pub(in crate::codegen::parser::types) fn parse_transfer_block(
        source: &'a str,
        token_stream: &mut TokenStream,
        context: &mut ParseContext,
    ) -> result::Result<Block<'a>> {
        // first token must be '@'
        let start_token = match token_stream.peek_token() {
            Some(token) => {
                if token.kind() != tokenizer::Kind::AT {
                    return Err(error::CompileError::from_parser(
                        source,
                        None,
                        "Expecting '@' token to start context extraction.",
                    ));
                }
                token
            }
            _ => {
                return Err(error::CompileError::from_parser(
                    source,
                    None,
                    "Empty token stream when expecting '@' token to start context extraction.",
                ));
            }
        };

        // consume @.
        token_stream.next_token();
        match token_stream.peek_token() {
            None => Err(error::CompileError::from_parser(
                source,
                Some(*start_token),
                "Expected content after '@'",
            )),
            Some(token) => {
                let block = match token.kind() {
                    tokenizer::Kind::OPARENTHESIS => {
                        // code part.
                        Self::parse_block_within_kinds(
                            source,
                            tokenizer::Kind::OPARENTHESIS,
                            tokenizer::Kind::CPARENTHESIS,
                            token_stream,
                            context,
                        )?
                    }
                    tokenizer::Kind::OCURLYBRACKET => {
                        // code part.
                        Self::parse_block_within_kinds(
                            source,
                            tokenizer::Kind::OCURLYBRACKET,
                            tokenizer::Kind::CCURLYBRACKET,
                            token_stream,
                            context,
                        )?
                    }
                    tokenizer::Kind::EXPRESSION => {
                        let exp = &source[token.range()];
                        // todo: create a map for directive keywords.
                        match exp {
                            consts::DIRECTIVE_KEYWORD_LAYOUT | consts::DIRECTIVE_KEYWORD_USE => {
                                Self::parse_directive(source, exp, token_stream)?
                            }
                            consts::KEYWORD_RENDER_SECTION => {
                                if context.is_code() {
                                    Self::parse_render(source, token_stream)?
                                } else {
                                    return Err(error::CompileError::from_parser(
                                        source,
                                        Some(*token),
                                        &format!(
                                            "'@{}' can only be used in content block.",
                                            consts::KEYWORD_RENDER_SECTION
                                        ),
                                    ));
                                }
                            }
                            consts::KEYWORD_SECTION => {
                                Self::parse_section(source, token, token_stream)?
                            }
                            _ => Self::create_inlined_code_block(
                                source,
                                token,
                                token_stream,
                                context,
                            )?,
                        }
                    }
                    tokenizer::Kind::ASTERISK => {
                        // comment part.
                        Self::parse_comment(source, start_token, token_stream)?
                    }
                    _ => {
                        return Err(error::CompileError::from_parser(
                            source,
                            Some(*token),
                            "Expected '(', '{' or expression after '@'",
                        ));
                    }
                };
                Ok(block)
            }
        }
    }

    fn create_inlined_code_block(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
        context: &ParseContext,
    ) -> result::Result<Block<'a>> {
        // consume the expression token.
        token_stream.next_token();
        let mut span = Span::new(source);
        span.push_token(*token);
        let block = ParseContext::create_block(&context, None, span)?;
        Ok(block)
    }
}
