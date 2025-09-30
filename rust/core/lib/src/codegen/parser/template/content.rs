use crate::codegen::consts;
use crate::codegen::parser::template::ParseContext;
use crate::codegen::parser::template::block::{self, Block};
use crate::codegen::parser::tokenizer::{self, Token, TokenStream};
use crate::types::{error, result};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    pub(crate) fn parse_section(
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
                            let mut context = ParseContext::new(block::Kind::SECTION);
                            let mut block = Self::parse_block_within_kinds(
                                source,
                                tokenizer::Kind::OCURLYBRACKET,
                                tokenizer::Kind::CCURLYBRACKET,
                                token_stream,
                                &mut context,
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
