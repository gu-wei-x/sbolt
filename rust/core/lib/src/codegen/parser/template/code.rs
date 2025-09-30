use crate::codegen::consts;
use crate::codegen::parser::template;
use crate::codegen::parser::template::block::Block;
use crate::codegen::parser::tokenizer;
use crate::codegen::parser::tokenizer::{Token, TokenStream};
use crate::types::error;
use crate::types::result;
use std::ops::Range;
use winnow::stream::Stream;

impl<'a> Block<'a> {
    // @exp, @{}, @()
    pub(crate) fn parse_at_block(
        source: &'a str,
        token_stream: &mut TokenStream,
        context: &mut template::ParseContext,
    ) -> result::Result<Block<'a>> {
        // first token must be '@'
        let start_token = match token_stream.peek_token() {
            Some(token) => {
                if token.kind() != tokenizer::Kind::AT {
                    return Err(error::Error::from_parser(
                        None,
                        "Expecting '@' token to start context extraction.",
                    ));
                }
                token
            }
            _ => {
                return Err(error::Error::from_parser(
                    None,
                    "Empty token stream when expecting '@' token to start context extraction.",
                ));
            }
        };

        // consume @.
        token_stream.next_token();
        match token_stream.peek_token() {
            None => Err(error::Error::from_parser(
                Some(*start_token),
                "Expected content after '@'",
            )),
            Some(token) => {
                let mut block = match token.kind() {
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
                        match exp {
                            consts::DIRECTIVE_KEYWORD_LAYOUT | consts::DIRECTIVE_KEYWORD_USE => {
                                Self::parse_directive(source, token, token_stream, exp)?
                            }
                            consts::KEYWORD_SECTION => {
                                Self::parse_section(source, token, token_stream)?
                            }
                            _ => {
                                // todo: consume until next transfer @, linefeed or whitespace for content.
                                let mut block =
                                    Self::create_inlined_code_block(source, token, token_stream)?;
                                block.with_kind(context.kind());
                                return Ok(block);
                            }
                        }
                    }
                    _ => {
                        return Err(error::Error::from_parser(
                            Some(*token),
                            "Expected '(', '{' or expression after '@'",
                        ));
                    }
                };
                block.with_kind(context.kind());
                Ok(block)
            }
        }
    }

    fn create_inlined_code_block(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        let content = &source[token.range()];
        let block = Block::new(None, token.range(), template::Kind::INLINEDCODE, content);

        // consume the expression token.
        token_stream.next_token();
        Ok(block)
    }

    fn parse_directive(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
        directive: &str,
    ) -> result::Result<Block<'a>> {
        // consume the layout token
        token_stream.next_token();

        // whitespace after layout token
        if !tokenizer::skip_whitespace(token_stream) {
            return Err(error::Error::from_parser(
                Some(*token),
                &format!("Expected whitespace name after '@{directive}'"),
            ));
        }

        match token_stream.peek_token() {
            None => Err(error::Error::from_parser(
                Some(*token),
                &format!("Expected {directive} content after '@{directive}'"),
            )),
            Some(start_token) => {
                let end_token = tokenizer::get_next_token_util(token_stream, |k| {
                    vec![tokenizer::Kind::SEMICOLON, tokenizer::Kind::NEWLINE].contains(&k)
                });

                match end_token {
                    None => Err(error::Error::from_parser(
                        Some(*token),
                        &format!("Expected {directive} content after '@{directive}'"),
                    )),
                    Some(end_token) => {
                        let start = start_token.range().start;
                        let end = end_token.range().start;
                        let exp = source[start..end].trim_end();
                        if exp.len() <= 0 {
                            return Err(error::Error::from_parser(
                                Some(*token),
                                &format!("Expected {directive} content after '@{directive}'"),
                            ));
                        }

                        let block = Block::new(
                            Some(directive.to_string()),
                            Range { start, end },
                            template::Kind::DIRECTIVE,
                            &exp,
                        );

                        // consume the end token.
                        token_stream.next_token();
                        Ok(block)
                    }
                }
            }
        }
    }
}
