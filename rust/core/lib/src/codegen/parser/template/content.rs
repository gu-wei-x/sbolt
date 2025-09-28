use crate::codegen::parser;
use crate::codegen::parser::template;
use crate::codegen::parser::template::block::Block;
use crate::codegen::parser::tokenizer::{self, Token, TokenStream, get_next_token_if};
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
                                        &source[token.range().start..end_token.range().start],
                                    ),
                                    start: token.range().start,
                                    end: end_token.range().start,
                                });
                            }
                        }
                        return Ok(block);
                    }
                    _ => Err(error::Error::from_parser(
                        Some(*token),
                        "Expected '(', '{' or expression after '@'",
                    )),
                }
            }
        }
    }
}
