use crate::codegen::parser::types::context::{Kind, ParseContext};
use crate::codegen::parser::types::util;
use crate::codegen::types::Span;
use crate::{
    codegen::{
        parser::tokenizer::{self, TokenStream},
        types::Block,
    },
    types::{error, result},
};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::types) fn parse(
        source: &'a str,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        tokenizer::skip_whitespace_and_newline(token_stream);
        match token_stream.peek_token() {
            None => {
                return Err(error::CompileError::from_parser(
                    source,
                    None,
                    "Empty stream",
                ));
            }
            Some(_) => {
                let mut context = ParseContext::new(Kind::KROOT);
                let mut span = Span::new(source);
                while let Some(token) = token_stream.peek_token() {
                    match token.kind() {
                        tokenizer::Kind::EOF => break,
                        tokenizer::Kind::NEWLINE => {
                            // todo: whether to ignore the lf
                            token_stream.next_token();
                            context.push(*token);
                        }
                        tokenizer::Kind::AT => {
                            match context.switch_if_possible(source, token_stream) {
                                Ok((true, mut new_context)) => {
                                    // 1. consume the current pending tokens belong to current context.
                                    if let Some(block) = context.consume(source)? {
                                        span.push_block(block);
                                    }

                                    // 2. switch context.
                                    let block = Block::parse_transfer_block(
                                        source,
                                        token_stream,
                                        &mut new_context,
                                    )?;
                                    span.push_block(block);
                                }
                                Ok((false, _)) => {
                                    if util::is_token_escaped(token_stream) {
                                        // consume 2 @ as one, only push one @ to context.
                                        token_stream.next_token();
                                    }
                                    token_stream.next_token();
                                    context.push(*token);
                                }
                                Err(e) => return Err(e),
                            }
                        }
                        _ => {
                            // consume and push to current context.
                            // todo: whether to ignore the lf
                            token_stream.next_token();
                            context.push(*token);
                        }
                    }
                }

                // consume the context.
                // todo: whether to ignore the lf
                if let Some(block) = context.consume(source)? {
                    span.push_block(block);
                }

                match span.has_blocks() {
                    false => Err(error::CompileError::from_parser(
                        source,
                        None,
                        "Empty block",
                    )),
                    true => {
                        let block = ParseContext::create_block(&context, None, span)?;
                        Ok(block)
                    }
                }
            }
        }
    }
}

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::types) fn parse_block_within_kinds(
        source: &'a str,
        open_kind: tokenizer::Kind,
        close_kind: tokenizer::Kind,
        token_stream: &mut TokenStream,
        context: &mut ParseContext,
    ) -> result::Result<Block<'a>> {
        // validate, first token must be open_kind and consume the token.
        let previous_token = token_stream.previous_tokens().last().copied();
        match token_stream.peek_token() {
            Some(token) => {
                if token.kind() != open_kind {
                    return Err(error::CompileError::from_parser(
                        source,
                        previous_token,
                        "Expected opening delimiter",
                    ));
                } else {
                    token_stream.next_token();
                }
            }
            _ => {
                return Err(error::CompileError::from_parser(
                    source,
                    previous_token,
                    "Expected opening delimiter",
                ));
            }
        }

        let mut depth = 1;
        let mut span = Span::new(source);
        while let Some(token) = token_stream.peek_token() {
            match token.kind() {
                k if k == open_kind => {
                    context.push(*token);
                    token_stream.next_token();
                    depth += 1;
                }
                k if k == close_kind => {
                    // should not push the closing delimiter to context.
                    // ignore the closing delimiter.
                    token_stream.next_token();
                    depth -= 1;
                    if depth == 0 {
                        break;
                    } else {
                        context.push(*token);
                    }
                }
                tokenizer::Kind::AT => {
                    if context.is_inline() {
                        return Err(error::CompileError::from_parser(
                            source,
                            Some(*token),
                            "Inlined block is not allowed to use '@' token",
                        ));
                    }

                    match context.switch_if_possible(source, token_stream) {
                        Ok((true, mut new_context)) => {
                            // 1. consume the current pending tokens belong to current context.
                            if let Some(block) = context.consume(source)? {
                                span.push_block(block);
                            }

                            // 2. switch context.
                            let block = Block::parse_transfer_block(
                                source,
                                token_stream,
                                &mut new_context,
                            )?;
                            span.push_block(block);
                        }
                        Ok((false, _)) => {
                            if util::is_token_escaped(token_stream) {
                                // consume 2 @ as one, only push one @ to context.
                                token_stream.next_token();
                            }
                            token_stream.next_token();
                            context.push(*token);
                        }
                        Err(e) => return Err(e),
                    }
                }
                _ => {
                    // consume and push to current context.
                    // TODO: for prettry, ignore the newline when generate code.
                    token_stream.next_token();
                    context.push(*token);
                }
            }
        }

        // not balanced.
        if depth != 0 {
            return Err(error::CompileError::from_parser(
                source,
                previous_token,
                "Unbalanced delimiters in block",
            ));
        }

        // consume the context.
        if let Some(block) = context.consume(source)? {
            span.push_block(block);
        }

        // if only one block, return the block directly.
        match span.blocks().len() {
            1 => {
                let only_block = span.blocks().into_iter().next().unwrap();
                Ok(only_block.clone())
            }
            _ => Ok(ParseContext::create_block(context, None, span)?),
        }
    }
}
