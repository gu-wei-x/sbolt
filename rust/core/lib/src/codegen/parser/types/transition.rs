use crate::codegen::consts;
use crate::codegen::parser::tokenizer::skip_newline;
use crate::codegen::parser::types::context::{Kind, ParseContext};
use crate::codegen::parser::{Token, tokenizer};
use crate::codegen::types::Span;
use crate::codegen::{parser::tokenizer::TokenStream, types::Block};
use crate::types::{error, result};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    // @exp, @{}, @()
    pub(in crate::codegen::parser::types) fn parse_transition_block<'s>(
        token_stream: &mut TokenStream,
        context: &mut ParseContext<'_, 's>,
    ) -> result::Result<Block<'s>> {
        // first token must be '@'
        let source = context.source();
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
                            tokenizer::Kind::OPARENTHESIS,
                            tokenizer::Kind::CPARENTHESIS,
                            token_stream,
                            context,
                        )?
                    }
                    tokenizer::Kind::OCURLYBRACKET => {
                        // code part.
                        Self::parse_block_within_kinds(
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
                            consts::KEYWORD_RENDER => {
                                if context.is_code() {
                                    Self::parse_render(
                                        token_stream,
                                        &mut context.clone_for(Kind::KRENDER),
                                    )?
                                } else {
                                    return Err(error::CompileError::from_parser(
                                        source,
                                        Some(*token),
                                        &format!(
                                            "'@{}' can only be used in content block.",
                                            consts::KEYWORD_RENDER
                                        ),
                                    ));
                                }
                            }
                            consts::KEYWORD_SECTION => Self::parse_section(
                                token,
                                token_stream,
                                &mut context.clone_for(Kind::KSECTION),
                            )?,
                            _ => Self::create_inlined_code_block(token, token_stream, context)?,
                        }
                    }
                    tokenizer::Kind::ASTERISK => {
                        // comment part.
                        Self::parse_comment(
                            start_token,
                            token_stream,
                            &mut context.clone_for(Kind::KCOMMENT),
                        )?
                    }
                    _ => {
                        return Err(error::CompileError::from_parser(
                            source,
                            Some(*token),
                            "Expected '(', '{' or expression after '@'",
                        ));
                    }
                };
                // skip the newline after the block.
                skip_newline(token_stream);
                Ok(block)
            }
        }
    }

    fn create_inlined_code_block<'s>(
        token: &Token,
        token_stream: &mut TokenStream,
        context: &ParseContext<'_, 's>,
    ) -> result::Result<Block<'s>> {
        // consume the expression token.
        token_stream.next_token();
        let mut span = Span::new(context.source());
        span.push_token(*token);
        let block = ParseContext::create_block(&context, None, span)?;
        Ok(block)
    }
}
