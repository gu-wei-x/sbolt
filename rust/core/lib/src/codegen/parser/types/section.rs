use crate::codegen::consts;
use crate::codegen::parser::Token;
use crate::codegen::parser::tokenizer::{self, TokenStream};
use crate::codegen::parser::types::context::{Kind, ParseContext};
use crate::codegen::types::{Block, Span};
use crate::types::{error, result};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::types) fn parse_section(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        // consume the section token
        token_stream.next_token();

        // whitespace after layout token
        if !tokenizer::skip_whitespace(token_stream) {
            return Err(error::CompileError::from_parser(
                source,
                Some(*token),
                &format!("Expected whitespace after '@{}'", consts::KEYWORD_SECTION),
            ));
        }

        match token_stream.peek_token() {
            None => Err(error::CompileError::from_parser(
                source,
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
                            // Note: section is content.
                            let block = Self::parse_block_within_kinds(
                                source,
                                tokenizer::Kind::OCURLYBRACKET,
                                tokenizer::Kind::CCURLYBRACKET,
                                token_stream,
                                &mut ParseContext::new(Kind::KSECTION),
                            )?;
                            let root_span = block.span();
                            let section_span = match root_span.is_simple() {
                                true => root_span.clone(),
                                false => {
                                    // unpack.
                                    let mut span = Span::new(source);
                                    for block in root_span.blocks() {
                                        // revert back
                                        if matches!(block, Block::KSECTION(_, _)) {
                                            span.push_block(block.to_content());
                                        } else {
                                            span.push_block(block.clone());
                                        }
                                    }
                                    span
                                }
                            };

                            let block = ParseContext::create_block(
                                &ParseContext::new(Kind::KSECTION),
                                Some(name.to_string()),
                                section_span,
                            )?;
                            Ok(block)
                        }
                        _ => Err(error::CompileError::from_parser(
                            source,
                            Some(*token),
                            &format!(
                                "Expected '{{' after '@{} {}'",
                                consts::KEYWORD_SECTION,
                                name
                            ),
                        )),
                    }
                }
                _ => Err(error::CompileError::from_parser(
                    source,
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
