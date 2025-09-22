use crate::codegen::parser;
use crate::codegen::parser::template;
use crate::codegen::parser::template::types::Block;
use crate::codegen::parser::tokenizer::{self, Token, TokenStream};
use crate::types::{error, result};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::template) fn parse_content(
        source: &'a str,
        start_token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        if start_token.kind() == tokenizer::Kind::AT {
            token_stream.next_token();
        }
        tokenizer::skip_whitespace(token_stream);
        let mut result = Block::default();
        let start_token2 = token_stream.peek_token();
        let mut start_token = token_stream.peek_token();
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
                        Block::create_content_block(source, &start_token, &Some(next_token))?;
                    result.push_block(content_block);

                    // 2. transfer to code.
                    let code_block = Block::parse_code(source, next_token, token_stream)?;
                    result.push_block(code_block);

                    // transfer back.
                    start_token = token_stream.peek_token();
                }
                _ => {
                    // content path.
                    token_stream.next_token();
                }
            }
        }

        match start_token {
            Some(token) => {
                let content_block = Self::create_content_block(source, &Some(token), &None)?;
                if start_token == start_token2 {
                    return Ok(content_block);
                } else {
                    result.push_block(content_block);
                }
            }
            None => { /* no-op */ }
        }

        Ok(result)
    }

    fn create_content_block(
        source: &'a str,
        start_token: &Option<&Token>,
        end_token: &Option<&Token>,
    ) -> result::Result<Block<'a>> {
        if start_token.is_none() {
            return Err(error::Error::from_parser("Missing start or end token"));
        }

        let start_token = start_token.unwrap();
        let start = start_token.range().start;
        let end = match end_token {
            Some(token) => token.range().start,
            None => source.len(),
        };

        if end <= start {
            return Err(error::Error::from_parser("Invalid token range"));
        }

        let mut block = Block::default();
        block.with_span(parser::Span {
            kind: template::Kind::CONTENT(&source[start..end]),
            start: start,
            end: end,
        });
        Ok(block)
    }
}
