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

        // consume the '@' token
        token_stream.next_token();
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
                        // inlined code part.
                        // caution: inlined should return to parent context.
                        // @section, @use, @layout
                        let exp = &source[token.range()];
                        match exp {
                            consts::KEYWORD_LAYOUT => {
                                Self::parse_layout(source, token, token_stream)
                            }
                            consts::KEYWORD_SECTION => {
                                Self::parse_section(source, token, token_stream)
                            }
                            consts::KEYWORD_USE => Self::parse_use(source, token, token_stream),
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
        token_stream.next_token();
        Ok(block)
    }

    fn parse_layout(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        // consume the layout token
        token_stream.next_token();

        // whitespace after layout token
        tokenizer::skip_whitespace(token_stream);
        match token_stream.peek_token() {
            None => Err(error::Error::from_parser(
                Some(*token),
                &format!(
                    "Expected {} name after '@{}'",
                    consts::KEYWORD_LAYOUT,
                    consts::KEYWORD_LAYOUT
                ),
            )),
            Some(start_token) => {
                // TODO: validate layout name
                let end_token = tokenizer::get_next_token_if(token_stream, |k| {
                    !vec![tokenizer::Kind::SEMICOLON, tokenizer::Kind::NEWLINE].contains(&k)
                });

                match end_token {
                    None => Err(error::Error::from_parser(
                        Some(*start_token),
                        &format!(
                            "Expected {} name after '@{}'",
                            consts::KEYWORD_LAYOUT,
                            consts::KEYWORD_LAYOUT
                        ),
                    )),
                    Some(end_token) => {
                        // TODO: validate layout name
                        let mut block = Block::default();
                        block.with_span(parser::Span {
                            kind: template::Kind::CODE(
                                &source[start_token.range().start..end_token.range().start],
                            ),
                            start: start_token.range().start,
                            end: end_token.range().start,
                        });
                        block.with_name(consts::KEYWORD_LAYOUT);
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

        // whitespace after section token
        tokenizer::skip_whitespace(token_stream);
        match token_stream.peek_token() {
            None => Err(error::Error::from_parser(
                Some(*token),
                &format!(
                    "Expected {} name after '@{}'",
                    consts::KEYWORD_SECTION,
                    consts::KEYWORD_SECTION
                ),
            )),
            Some(start_token) => {
                match start_token.kind() {
                    tokenizer::Kind::EXPRESSION => {
                        // TODO: validate section name
                        let section_name = &source[start_token.range()];
                        token_stream.next_token();
                        tokenizer::skip_whitespace(token_stream);
                        let next_token = token_stream.peek_token();
                        match next_token {
                            Some(token) if token.kind() == tokenizer::Kind::OCURLYBRACKET => {
                                // code part.
                                let mut block = Self::parse_block_within_kind(
                                    source,
                                    tokenizer::Kind::OCURLYBRACKET,
                                    tokenizer::Kind::CCURLYBRACKET,
                                    token_stream,
                                    false,
                                    false,
                                )?;
                                block.with_name(section_name);
                                Ok(block)
                            }
                            _ => Err(error::Error::from_parser(
                                Some(*start_token),
                                &format!(
                                    "Expected '{{' after section name for '@{}'",
                                    consts::KEYWORD_SECTION
                                ),
                            )),
                        }
                    }
                    _ => {
                        return Err(error::Error::from_parser(
                            Some(*start_token),
                            &format!(
                                "Expected {} name after '@{}'",
                                consts::KEYWORD_SECTION,
                                consts::KEYWORD_SECTION
                            ),
                        ));
                    }
                }
            }
        }
    }

    fn parse_use(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        // consume the use token
        token_stream.next_token();

        // whitespace after use token
        tokenizer::skip_whitespace(token_stream);
        let end_token = tokenizer::get_next_token_if(token_stream, |k| {
            !vec![tokenizer::Kind::SEMICOLON, tokenizer::Kind::NEWLINE].contains(&k)
        });

        match end_token {
            None => Err(error::Error::from_parser(
                Some(*token),
                &format!("Expected module path after '@{}'", consts::KEYWORD_USE),
            )),
            Some(end_token) => {
                let start = token.range().start;
                let end = end_token.range().start;
                let exp = source[start..end].trim_end();
                if exp.len() <= consts::KEYWORD_USE.len() {
                    return Err(error::Error::from_parser(
                        Some(*token),
                        &format!("Expected module path after '@{}'", consts::KEYWORD_USE),
                    ));
                }
                let mut block = Block::default();
                block.with_span(parser::Span {
                    kind: template::Kind::CODE(exp),
                    start: start,
                    end: end,
                });
                block.with_name(consts::KEYWORD_USE);
                token_stream.next_token();
                Ok(block)
            }
        }
    }
}
