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

    pub(crate) fn parse_comment(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        let mut result = Block::new(None, block::Kind::COMMENT, source);
        result.push_token(*token); // push '@' token

        // note: @ was consumed before calling this function.
        // the first is * token
        match token_stream.peek_token() {
            Some(t) if t.kind() == tokenizer::Kind::ASTERISK => {
                // consume the '*' token
                result.push_token(*t);
                token_stream.next_token();
            }
            _ => {
                return Err(error::CompileError::from_parser(
                    source,
                    Some(*token),
                    "Expected '*' after '@' for comment block",
                ));
            }
        }

        // Consume tokens until we find the closing '*@'
        let mut is_ended = false;
        while let Some(tok) = token_stream.peek_token() {
            match tok.kind() {
                tokenizer::Kind::ASTERISK => {
                    // consume the '*' token
                    result.push_token(*tok);
                    token_stream.next_token();

                    // check next token
                    if let Some(next_tok) = token_stream.peek_token() {
                        if next_tok.kind() == tokenizer::Kind::AT {
                            // Found the closing '@', consume '@' and exit the loop
                            result.push_token(*next_tok);
                            token_stream.next_token(); // consume '@'
                            is_ended = true;
                            break;
                        } else {
                            if next_tok.kind() != tokenizer::Kind::ASTERISK {
                                // consume the token after '*' if it's not another '*'
                                result.push_token(*next_tok);
                                token_stream.next_token();
                                continue;
                            }
                        }
                    } else {
                        // No more tokens after '*', unterminated comment
                        return Err(error::CompileError::from_parser(
                            source,
                            Some(*token),
                            "Unterminated comment block, expected '*@'",
                        ));
                    }
                }
                _ => {
                    result.push_token(*tok);
                    token_stream.next_token();
                    continue;
                }
            }
        }

        match is_ended {
            true => Ok(result),
            false => {
                // If we reach here, we didn't find a closing '*@'
                Err(error::CompileError::from_parser(
                    source,
                    Some(*token),
                    "Unterminated comment block, expected '*@'",
                ))
            }
        }
    }
}
