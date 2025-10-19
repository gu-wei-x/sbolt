use crate::codegen::parser::Token;
use crate::codegen::parser::tokenizer::{self, TokenStream};
use crate::codegen::parser::types::context::ParseContext;
use crate::codegen::types::Block;
use crate::types::{error, result};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::types) fn parse_comment<'s>(
        token: &Token,
        token_stream: &mut TokenStream,
        context: &mut ParseContext<'_, 's>,
    ) -> result::Result<Block<'s>> {
        let source = context.source();
        context.push(*token);

        // note: @ was consumed before calling this function.
        // the first is * token
        match token_stream.peek_token() {
            Some(t) if t.kind() == tokenizer::Kind::ASTERISK => {
                // consume the '*' token
                context.push(*t);
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
                    context.push(*tok);
                    token_stream.next_token();

                    // check next token
                    if let Some(next_tok) = token_stream.peek_token() {
                        if next_tok.kind() == tokenizer::Kind::AT {
                            // Found the closing '@', consume '@' and exit the loop
                            context.push(*next_tok);
                            token_stream.next_token(); // consume '@'
                            is_ended = true;
                            break;
                        } else {
                            if next_tok.kind() != tokenizer::Kind::ASTERISK {
                                // consume the token after '*' if it's not another '*'
                                context.push(*next_tok);
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
                    context.push(*tok);
                    token_stream.next_token();
                    continue;
                }
            }
        }

        if !is_ended {
            return Err(error::CompileError::from_parser(
                source,
                Some(*token),
                "Unterminated comment block, expected '*@'",
            ));
        }

        match context.consume()? {
            Some(block) => Ok(block),
            _ => Err(error::CompileError::from_parser(
                source,
                Some(*token),
                "Unbale to parse comment block",
            )),
        }
    }
}
