use crate::codegen::parser;
use crate::codegen::parser::template;
use crate::codegen::parser::template::block::Block;
use crate::codegen::parser::tokenizer::{self, Token, TokenStream, get_next_token_if};
use crate::types::{error, result};
use winnow::stream::Stream as _;
impl<'a> Block<'a> {
    pub(in crate::codegen::parser::template) fn parse_content(
        source: &'a str,
        start_token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        match start_token.kind() {
            tokenizer::Kind::AT => {
                // from code to content.
                tokenizer::skip_whitespace(token_stream);
                while let Some(next_token) = token_stream.peek_token() {
                    match next_token.kind() {
                        tokenizer::Kind::EOF => break,
                        tokenizer::Kind::OPARENTHESIS => {
                            // content inside ().
                            return Self::parse_block_within_kind(
                                source,
                                tokenizer::Kind::OPARENTHESIS,
                                tokenizer::Kind::CPARENTHESIS,
                                true,
                                token_stream,
                            );
                        }
                        tokenizer::Kind::OCURLYBRACKET => {
                            // content inside {}.
                            return Self::parse_block_within_kind(
                                source,
                                tokenizer::Kind::OCURLYBRACKET,
                                tokenizer::Kind::CCURLYBRACKET,
                                true,
                                token_stream,
                            );
                        }
                        _ => {
                            // inlined: consume until line break and return to code context.
                            let end_token = get_next_token_if(token_stream, |k| {
                                !vec![tokenizer::Kind::WHITESPACE, tokenizer::Kind::NEWLINE]
                                    .contains(&k)
                            });
                            let mut block = Block::default();
                            match end_token {
                                None => {
                                    // end of file.
                                    block.with_span(parser::Span {
                                        kind: template::Kind::CONTENT(
                                            &source[next_token.range().start..source.len()],
                                        ),
                                        start: next_token.range().start,
                                        end: source.len(),
                                    });
                                }
                                Some(token) => {
                                    block.with_span(parser::Span {
                                        kind: template::Kind::CONTENT(
                                            &source[next_token.range().start..token.range().start],
                                        ),
                                        start: next_token.range().start,
                                        end: token.range().start,
                                    });
                                }
                            }
                            return Ok(block);
                        }
                    }
                }
                return Err(error::Error::from_parser(
                    Some(*start_token),
                    "Single @ must be followed by a block or inline content.",
                ));
            }
            _ => {
                // content.
                return Self::parse_content_block(source, start_token, token_stream);
            }
        }
    }
}

impl<'a> Block<'a> {
    fn parse_content_block(
        source: &'a str,
        start_token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        if start_token.kind() == tokenizer::Kind::AT {
            token_stream.next_token();
        }
        tokenizer::skip_whitespace(token_stream);
        let mut result = Block::default();
        let start = token_stream.peek_token();
        let mut next_start = token_stream.peek_token();
        while let Some(next_token) = token_stream.peek_token() {
            match next_token.kind() {
                tokenizer::Kind::EOF => break,
                tokenizer::Kind::NEWLINE => {
                    // consume newline.
                    token_stream.next_token();
                }
                tokenizer::Kind::AT => {
                    // 1. consume the tokens before this one as content block and switch to code block.
                    // transfer to code.
                    let content_block =
                        Block::create_block(source, &next_start, &Some(next_token), true)?;
                    result.push_block(content_block);

                    // 2. transfer to code.
                    let code_block = Block::parse_code(source, next_token, token_stream)?;
                    result.push_block(code_block);

                    // transfer back.
                    next_start = token_stream.peek_token();
                }
                _ => {
                    // content path.
                    token_stream.next_token();
                }
            }
        }

        match next_start {
            Some(token) => {
                let content_block = Self::create_block(source, &Some(token), &None, true)?;
                if next_start == start {
                    return Ok(content_block);
                } else {
                    result.push_block(content_block);
                }
            }
            None => { /* no-op */ }
        }

        Ok(result)
    }
}
