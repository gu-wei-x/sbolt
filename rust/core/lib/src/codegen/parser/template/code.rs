use crate::codegen::parser;
use crate::codegen::parser::template;
use crate::codegen::parser::template::types::Block;
use crate::codegen::parser::tokenizer::{self, Token, TokenStream};
use crate::types::error; // Add this import if your error type is defined here
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
                    return Self::parse_code_block_with_kind(
                        source,
                        tokenizer::Kind::OPARENTHESIS,
                        tokenizer::Kind::CPARENTHESIS,
                        token_stream,
                    );
                }
                tokenizer::Kind::OCURLYBRACKET => {
                    // code part.
                    let code_block = Self::parse_code_block_with_kind(
                        source,
                        tokenizer::Kind::OCURLYBRACKET,
                        tokenizer::Kind::CCURLYBRACKET,
                        token_stream,
                    );
                    return code_block;
                }
                tokenizer::Kind::EXPRESSION => {
                    // inlined code part.
                    let code_block =
                        Self::create_inlined_code_block(source, next_token, token_stream);
                    // caution: inlined should return to parent context.
                    return code_block;
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

    fn create_code_block(
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
            kind: template::Kind::CODE(&source[start..end]),
            start: start,
            end: end,
        });
        Ok(block)
    }

    fn parse_code_block_with_kind(
        source: &'a str,
        open_kind: tokenizer::Kind,
        close_kind: tokenizer::Kind,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        // Assume the current token is the opening delimiter (either '(' or '{')
        _ = token_stream
            .next_token()
            .ok_or_else(|| error::Error::from_parser("Expected opening delimiter").into())?;

        let mut depth = 1;
        let mut result = Block::default();
        let start_token = token_stream.peek_token();
        let mut start_token2 = token_stream.peek_token();
        let mut end_token: Option<&Token> = None;
        while let Some(token) = token_stream.next_token() {
            match token.kind() {
                k if k == open_kind => {
                    depth += 1;
                }
                k if k == close_kind => {
                    depth -= 1;
                    if depth == 0 {
                        end_token = Some(token);
                        break;
                    }
                }
                tokenizer::Kind::AT => {
                    // 1. consume the tokens before this one as code block and switch to content block.
                    let code_block = Self::create_code_block(source, &start_token2, &Some(token))?;
                    result.push_block(code_block);

                    // 2. transfer to content.
                    let content_block = Self::parse_content(source, token, token_stream)?;
                    result.push_block(content_block);

                    // transfer back.
                    start_token2 = token_stream.peek_token();
                }
                _ => {}
            }
        }

        if depth != 0 {
            return error::Error::from_parser("Unbalanced delimiters in code block").into();
        }

        match start_token2 {
            Some(token) => {
                let code_block = Self::create_code_block(source, &Some(token), &end_token)?;
                if start_token == start_token2 {
                    return Ok(code_block);
                } else {
                    result.push_block(code_block);
                }
            }
            None => { /* no-op */ }
        }

        Ok(result)
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
