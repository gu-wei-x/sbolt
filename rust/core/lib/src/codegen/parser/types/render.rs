#![allow(dead_code)]
use crate::codegen::consts;
use crate::codegen::parser::tokenizer::{self, TokenStream, get_nth_token};
use crate::codegen::parser::types::context::Kind;
use crate::codegen::parser::types::context::ParseContext;
use crate::codegen::parser::types::util::{get_token_if, get_tokens_before};
use crate::codegen::types::Block;
use crate::codegen::types::Span;
use crate::types::{error, result};
use winnow::stream::{Stream as _, TokenSlice};

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::types) fn parse_render(
        source: &'a str,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        // token_stream starts with the directive.
        let start_token = match token_stream.peek_token() {
            Some(token) => match token.kind() {
                tokenizer::Kind::EXPRESSION if consts::KEYWORD_RENDER == &source[token.range()] => {
                    token
                }
                _ => {
                    return Err(error::CompileError::from_parser(
                        source,
                        Some(*token),
                        &format!("Expected '{}' after '@'", consts::KEYWORD_RENDER),
                    ));
                }
            },
            None => {
                return Err(error::CompileError::from_parser(
                    source,
                    None,
                    &format!("Expected '{}' after '@'", consts::KEYWORD_RENDER),
                ));
            }
        };

        // render(), render(exp, true|false)
        // consume the directive token
        token_stream.next_token();
        tokenizer::skip_whitespace(token_stream);

        // next must be '('
        get_token_if(token_stream, |k| k == tokenizer::Kind::OPARENTHESIS).ok_or_else(|| {
            error::CompileError::from_parser(
                source,
                Some(*start_token),
                &format!("Expected '(' after '{}'", consts::KEYWORD_RENDER,),
            )
        })?;
        tokenizer::skip_whitespace(token_stream);
        let tokens = get_tokens_before(token_stream, |kind| {
            vec![tokenizer::Kind::COMMA, tokenizer::Kind::CPARENTHESIS].contains(&kind)
        })
        .ok_or_else(|| {
            error::CompileError::from_parser(
                source,
                Some(*start_token),
                &format!(
                    "Expected parameters after '(' for '{}'",
                    consts::KEYWORD_RENDER,
                ),
            )
        })?;

        let mut span = Span::new(source);
        // no params @render().
        if tokens.len() == 0 {
            let block = ParseContext::create_block(&ParseContext::new(Kind::KRENDER), None, span)?;
            return Ok(block);
        }

        let left_block = Block::parse(source, &mut TokenSlice::new(&tokens))?;
        span.push_block(left_block);

        let next_token = get_nth_token(token_stream, 0);
        if let Some(token) = next_token {
            if token.kind() == tokenizer::Kind::CPARENTHESIS {
                // only one param
                token_stream.next_token();
                let block =
                    ParseContext::create_block(&ParseContext::new(Kind::KRENDER), None, span)?;
                return Ok(block);
            } else if token.kind() != tokenizer::Kind::COMMA {
                return Err(error::CompileError::from_parser(
                    source,
                    Some(*token),
                    &format!(
                        "Expected ',' or ')' after first parameter for '{}'",
                        consts::KEYWORD_RENDER,
                    ),
                ));
            }
        }

        // skip ,
        token_stream.next_token();
        tokenizer::skip_whitespace(token_stream);

        let tokens = get_tokens_before(token_stream, |kind| {
            vec![tokenizer::Kind::CPARENTHESIS].contains(&kind)
        })
        .ok_or_else(|| {
            error::CompileError::from_parser(
                source,
                Some(*start_token),
                &format!("Expected second parameter for '{}'", consts::KEYWORD_RENDER,),
            )
        })?;
        let right_block = Block::parse(source, &mut TokenSlice::new(&tokens))?;
        span.push_block(right_block);

        // skip )
        token_stream.next_token();
        let block = ParseContext::create_block(&ParseContext::new(Kind::KRENDER), None, span)?;
        Ok(block)
    }
}
