use crate::codegen::parser;
use crate::codegen::parser::template;
use crate::codegen::parser::template::block::Block;
use crate::codegen::parser::tokenizer::{self, Token, TokenStream};
use crate::types::error;
use crate::types::result;
use winnow::stream::Stream;

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::template) fn parse_code(
        source: &'a str,
        start_token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        if start_token.kind() != tokenizer::Kind::AT {
            return error::Error::from_parser("Expected '@' token").into();
        }

        token_stream.next_token();
        tokenizer::skip_whitespace(token_stream);
        while let Some(next_token) = token_stream.peek_token() {
            match next_token.kind() {
                tokenizer::Kind::EOF => break,
                tokenizer::Kind::NEWLINE => {
                    // consume newline.
                    token_stream.next_token();
                }
                tokenizer::Kind::OPARENTHESIS => {
                    // code part.
                    return Self::parse_block_within_kind(
                        source,
                        tokenizer::Kind::OPARENTHESIS,
                        tokenizer::Kind::CPARENTHESIS,
                        false,
                        token_stream,
                    );
                }
                tokenizer::Kind::OCURLYBRACKET => {
                    // code part.
                    return Self::parse_block_within_kind(
                        source,
                        tokenizer::Kind::OCURLYBRACKET,
                        tokenizer::Kind::CCURLYBRACKET,
                        false,
                        token_stream,
                    );
                }
                tokenizer::Kind::EXPRESSION => {
                    // inlined code part.
                    // caution: inlined should return to parent context.
                    return Self::create_inlined_code_block(source, next_token, token_stream);
                }
                _ => {
                    // code part.
                    //token_stream.next_token();
                    return Err(error::Error::from_parser("Failed to parse code block").into());
                }
            }
        }

        Err(error::Error::from_parser("Failed to parse code block").into())
    }

    fn create_inlined_code_block(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        let mut block = Block::default();
        block.with_span(parser::Span {
            kind: template::Kind::CODE(&source[token.range()]),
            start: token.range().start,
            end: token.range().end,
        });
        token_stream.next_token();
        Ok(block)
    }
}
