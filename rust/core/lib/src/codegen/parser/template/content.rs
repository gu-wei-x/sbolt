use crate::codegen::parser::template;
use crate::codegen::parser::template::block::Block;
use crate::codegen::parser::tokenizer::{self, Token, TokenStream, get_next_token_if};
use crate::codegen::{consts, parser};
use crate::types::{error, result};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    pub(crate) fn parse_content(
        source: &'a str,
        start_token: &Token,
        token_stream: &mut TokenStream,
        is_inlined: bool,
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
                    tokenizer::Kind::OPARENTHESIS if !is_inlined => {
                        // inlinded.
                        Self::parse_block_within_kind(
                            source,
                            tokenizer::Kind::OPARENTHESIS,
                            tokenizer::Kind::CPARENTHESIS,
                            token_stream,
                            true,
                            true,
                        )
                    }
                    tokenizer::Kind::OCURLYBRACKET => Self::parse_block_within_kind(
                        source,
                        tokenizer::Kind::OCURLYBRACKET,
                        tokenizer::Kind::CCURLYBRACKET,
                        token_stream,
                        true,
                        false,
                    ),
                    tokenizer::Kind::EXPRESSION => {
                        let exp = &source[token.range()];
                        match exp {
                            consts::KEYWORD_SECTION => {
                                Self::parse_section(source, token, token_stream)
                            }
                            _ => {
                                // TODO: consume util next transfer @, linefeed, @layout.
                                let end_token = get_next_token_if(token_stream, |k| {
                                    !vec![tokenizer::Kind::WHITESPACE, tokenizer::Kind::NEWLINE]
                                        .contains(&k)
                                });
                                let mut block = Block::default();
                                match end_token {
                                    None => {
                                        // end of file.
                                        block.with_span(parser::Span {
                                            kind: template::Kind::INLINEDCONTENT(
                                                &source[token.range().start..source.len()],
                                            ),
                                            start: token.range().start,
                                            end: source.len(),
                                        });
                                    }
                                    Some(end_token) => {
                                        block.with_span(parser::Span {
                                            kind: template::Kind::INLINEDCONTENT(
                                                &source
                                                    [token.range().start..end_token.range().start],
                                            ),
                                            start: token.range().start,
                                            end: end_token.range().start,
                                        });
                                    }
                                }
                                Ok(block)
                            }
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
