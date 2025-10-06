#![allow(dead_code)]
use crate::codegen::parser::Token;
use crate::codegen::parser::tokenizer::{self, TokenStream};
use crate::codegen::parser::types::context::{Kind, ParseContext};
use crate::codegen::types::Block;
use crate::codegen::types::Span;
use crate::types::{error, result};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    pub(in crate::codegen) fn parse_comment(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        let mut result = Span::new(source);
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
            true => Ok(ParseContext::create_block(
                &ParseContext::new(Kind::KCOMMENT),
                None,
                result,
            )?),
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
