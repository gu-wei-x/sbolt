use crate::codegen::consts;
use crate::codegen::parser::tokenizer::{self, TokenStream};
use crate::codegen::parser::types::context::Kind;
use crate::codegen::parser::types::context::ParseContext;
use crate::codegen::parser::types::util::get_token_if;
use crate::codegen::types::Block;
use crate::codegen::types::Span;
use crate::types::{error, result};
use winnow::stream::Stream as _;

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

        // render, render(exp), render(exp, true|false)
        // consume the directive token
        token_stream.next_token();
        tokenizer::skip_whitespace(token_stream);

        // TODO(future): p1 is block, p2 is also block, see previous commit.
        // without ()
        if let None = get_token_if(token_stream, |k| k == tokenizer::Kind::OPARENTHESIS) {
            // empty render
            let block = ParseContext::create_block(
                &ParseContext::new(Kind::KRENDER),
                None,
                Span::new(source),
            )?;
            return Ok(block);
        }
        tokenizer::skip_whitespace(token_stream);

        let token = get_token_if(token_stream, |k| {
            k == tokenizer::Kind::EXPRESSION || k == tokenizer::Kind::CPARENTHESIS
        })
        .ok_or_else(|| {
            error::CompileError::from_parser(
                source,
                Some(*start_token),
                &format!("Expected '(' after '{}'", consts::KEYWORD_RENDER,),
            )
        })?;

        match token.kind() {
            tokenizer::Kind::CPARENTHESIS => {
                // no params @render()
                // empty render
                let block = ParseContext::create_block(
                    &ParseContext::new(Kind::KRENDER),
                    None,
                    Span::new(source),
                )?;
                return Ok(block);
            }
            tokenizer::Kind::EXPRESSION => {
                let mut root_span = Span::new(source);

                // has params
                let mut left_span = Span::new(source);
                left_span.push_token(*token);
                let left_block = ParseContext::create_block(
                    &ParseContext::new(Kind::KCONTENT),
                    None,
                    left_span,
                )?;
                root_span.push_block(left_block);

                tokenizer::skip_whitespace(token_stream);
                tokenizer::skip_next_token_if(token_stream, |k| k == tokenizer::Kind::COMMA);
                tokenizer::skip_whitespace(token_stream);
                // second.
                let token = get_token_if(token_stream, |k| {
                    k == tokenizer::Kind::EXPRESSION || k == tokenizer::Kind::CPARENTHESIS
                })
                .ok_or_else(|| {
                    error::CompileError::from_parser(
                        source,
                        Some(*start_token),
                        &format!("Expected '(' after '{}'", consts::KEYWORD_RENDER,),
                    )
                })?;

                match token.kind() {
                    tokenizer::Kind::CPARENTHESIS => {
                        // only one param
                        let block = ParseContext::create_block(
                            &ParseContext::new(Kind::KRENDER),
                            None,
                            root_span,
                        )?;
                        return Ok(block);
                    }
                    tokenizer::Kind::EXPRESSION => {
                        // validate, must be true or false
                        let text = &source[token.range()];
                        let is_bool = text.parse::<bool>().is_ok();
                        if !is_bool {
                            return Err(error::CompileError::from_parser(
                                source,
                                Some(*token),
                                &format!(
                                    "Expected boolean literal (true or false) as second parameter for '{}'",
                                    consts::KEYWORD_RENDER,
                                ),
                            ));
                        }

                        // has two params
                        let mut right_span = Span::new(source);
                        right_span.push_token(*token);
                        let right_block = ParseContext::create_block(
                            &ParseContext::new(Kind::KCONTENT),
                            None,
                            right_span,
                        )?;
                        root_span.push_block(right_block);
                        tokenizer::skip_whitespace(token_stream);
                        // must be )
                        get_token_if(token_stream, |k| k == tokenizer::Kind::CPARENTHESIS)
                            .ok_or_else(|| {
                                error::CompileError::from_parser(
                                    source,
                                    Some(*start_token),
                                    &format!(
                                        "Expected ')' after second parameter for '{}'",
                                        consts::KEYWORD_RENDER,
                                    ),
                                )
                            })?;
                        Ok(ParseContext::create_block(
                            &ParseContext::new(Kind::KRENDER),
                            None,
                            root_span,
                        )?)
                    }
                    _ => {
                        return Err(error::CompileError::from_parser(
                            source,
                            Some(*token),
                            &format!(
                                "Expected parameters or ')' after '(' for '{}'",
                                consts::KEYWORD_RENDER,
                            ),
                        ));
                    }
                }
            }
            _ => {
                return Err(error::CompileError::from_parser(
                    source,
                    Some(*token),
                    &format!(
                        "Expected parameters or ')' after '(' for '{}'",
                        consts::KEYWORD_RENDER,
                    ),
                ));
            }
        }
    }
}
