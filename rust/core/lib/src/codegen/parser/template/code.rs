use crate::codegen::consts;
use crate::codegen::parser;
use crate::codegen::parser::template;
use crate::codegen::parser::template::block::Block;
use crate::codegen::parser::tokenizer;
use crate::codegen::parser::tokenizer::{Token, TokenStream};
use crate::types::error;
use crate::types::result;
use winnow::stream::Stream;

impl<'a> Block<'a> {
    // @exp, @{}, @()
    pub(crate) fn parse_code(
        source: &'a str,
        start_token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        if start_token.kind() != tokenizer::Kind::AT {
            return Err(error::Error::from_parser(
                Some(*start_token),
                "Expected '@'",
            ));
        }

        if Some(start_token) == token_stream.peek_token() {
            // consume @.
            token_stream.next_token();
        }
        match token_stream.peek_token() {
            None => Err(error::Error::from_parser(
                Some(*start_token),
                "Expected content after '@'",
            )),
            Some(token) => {
                match token.kind() {
                    tokenizer::Kind::OPARENTHESIS => {
                        // code part.
                        Self::parse_block_within_kind(
                            source,
                            tokenizer::Kind::OPARENTHESIS,
                            tokenizer::Kind::CPARENTHESIS,
                            token_stream,
                            false,
                            true,
                        )
                    }
                    tokenizer::Kind::OCURLYBRACKET => {
                        // code part.
                        Self::parse_block_within_kind(
                            source,
                            tokenizer::Kind::OCURLYBRACKET,
                            tokenizer::Kind::CCURLYBRACKET,
                            token_stream,
                            false,
                            false,
                        )
                    }
                    tokenizer::Kind::EXPRESSION => {
                        let exp = &source[token.range()];
                        match exp {
                            consts::DIRECTIVE_KEYWORD_LAYOUT | consts::DIRECTIVE_KEYWORD_USE => {
                                Self::parse_directive(source, token, token_stream, exp)
                            }
                            consts::KEYWORD_SECTION => {
                                Self::parse_section(source, token, token_stream)
                            }
                            _ => Self::create_inlined_code_block(source, token, token_stream),
                        }
                    }
                    _ => Err(error::Error::from_parser(
                        Some(*token),
                        "Expected '(', '{' or expression after '@'",
                    )),
                }
            }
        }
    }

    fn create_inlined_code_block(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        let mut block = Block::default();
        block.with_span(parser::Span {
            kind: template::Kind::INLINEDCODE(&source[token.range()]),
            start: token.range().start,
            end: token.range().end,
        });

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
                        let mut block = Block::default();
                        block.with_span(parser::Span {
                            kind: template::Kind::CODE(&exp),
                            start: start,
                            end: end,
                        });
                        block.with_name(directive);

                        // consume the end token.
                        token_stream.next_token();
                        Ok(block)
                    }
                }
            }
        }
    }

    fn parse_section(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        // consume the section token
        token_stream.next_token();

        // whitespace after layout token
        if !tokenizer::skip_whitespace(token_stream) {
            return Err(error::Error::from_parser(
                Some(*token),
                &format!("Expected whitespace after '@{}'", consts::KEYWORD_SECTION),
            ));
        }

        match token_stream.peek_token() {
            None => Err(error::Error::from_parser(
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
                            let mut block = Self::parse_block_within_kind(
                                source,
                                tokenizer::Kind::OCURLYBRACKET,
                                tokenizer::Kind::CCURLYBRACKET,
                                token_stream,
                                true,
                                false,
                            )?;
                            block.with_name(name);
                            Ok(block)
                        }
                        _ => Err(error::Error::from_parser(
                            Some(*token),
                            &format!(
                                "Expected '{{' after '@{} {}'",
                                consts::KEYWORD_SECTION,
                                name
                            ),
                        )),
                    }
                }
                _ => Err(error::Error::from_parser(
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
