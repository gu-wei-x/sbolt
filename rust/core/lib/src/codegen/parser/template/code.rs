use crate::codegen::consts;
use crate::codegen::parser::template;
use crate::codegen::parser::template::block::Block;
use crate::codegen::parser::tokenizer::{self, get_nth_token};
use crate::codegen::parser::tokenizer::{Token, TokenStream};
use crate::types::error;
use crate::types::result;
use winnow::stream::Stream;

impl<'a> Block<'a> {
    // @exp, @{}, @()
    pub(crate) fn parse_at_block(
        source: &'a str,
        token_stream: &mut TokenStream,
        context: &mut template::ParseContext,
    ) -> result::Result<Block<'a>> {
        // first token must be '@'
        let start_token = match token_stream.peek_token() {
            Some(token) => {
                if token.kind() != tokenizer::Kind::AT {
                    return Err(error::CompileError::from_parser(
                        None,
                        "Expecting '@' token to start context extraction.",
                    ));
                }
                token
            }
            _ => {
                return Err(error::CompileError::from_parser(
                    None,
                    "Empty token stream when expecting '@' token to start context extraction.",
                ));
            }
        };

        // consume @.
        token_stream.next_token();
        match token_stream.peek_token() {
            None => Err(error::CompileError::from_parser(
                Some(*start_token),
                "Expected content after '@'",
            )),
            Some(token) => {
                let mut block = match token.kind() {
                    tokenizer::Kind::OPARENTHESIS => {
                        // code part.
                        Self::parse_block_within_kinds(
                            source,
                            tokenizer::Kind::OPARENTHESIS,
                            tokenizer::Kind::CPARENTHESIS,
                            token_stream,
                            context,
                        )?
                    }
                    tokenizer::Kind::OCURLYBRACKET => {
                        // code part.
                        Self::parse_block_within_kinds(
                            source,
                            tokenizer::Kind::OCURLYBRACKET,
                            tokenizer::Kind::CCURLYBRACKET,
                            token_stream,
                            context,
                        )?
                    }
                    tokenizer::Kind::EXPRESSION => {
                        let exp = &source[token.range()];
                        match exp {
                            consts::DIRECTIVE_KEYWORD_LAYOUT | consts::DIRECTIVE_KEYWORD_USE => {
                                Self::parse_directive(source, token, token_stream, exp)?
                            }
                            consts::KEYWORD_SECTION => {
                                Self::parse_section(source, token, token_stream)?
                            }
                            _ => {
                                // todo: consume until next transfer @, linefeed or whitespace for content.
                                let mut block =
                                    Self::create_inlined_code_block(source, token, token_stream)?;
                                block.with_kind(context.kind());
                                return Ok(block);
                            }
                        }
                    }
                    tokenizer::Kind::ASTERISK => {
                        // comment part.
                        Self::parse_comment(source, start_token, token_stream)?
                    }
                    _ => {
                        return Err(error::CompileError::from_parser(
                            Some(*token),
                            "Expected '(', '{' or expression after '@'",
                        ));
                    }
                };
                block.with_kind(context.kind());
                Ok(block)
            }
        }
    }

    fn create_inlined_code_block(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        let mut result = Block::new(None, template::Kind::INLINEDCODE, source);
        result.push_token(*token);

        // consume the expression token.
        token_stream.next_token();
        Ok(result)
    }

    fn parse_directive(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
        directive: &str,
    ) -> result::Result<Block<'a>> {
        // consume the directive token
        token_stream.next_token();

        // whitespace after directive token
        if !tokenizer::skip_whitespace(token_stream) {
            return Err(error::CompileError::from_parser(
                Some(*token),
                &format!("Expected whitespace name after '@{directive}'"),
            ));
        }

        // validate directive content.
        let next_token = get_nth_token(token_stream, 0);
        if None == next_token {
            return Err(error::CompileError::from_parser(
                Some(*token),
                &format!("Expected {directive} content after '@{directive}'"),
            ));
        }

        let mut result = Block::new(
            Some(directive.to_string()),
            template::Kind::DIRECTIVE,
            source,
        );
        while let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::NEWLINE | tokenizer::Kind::SEMICOLON => {
                    // consume the end token.
                    token_stream.next_token();
                    break;
                }
                _ => {
                    result.push_token(*token);
                    token_stream.next_token();
                }
            }
        }

        let content = result.content();
        if content.trim().is_empty() {
            return Err(error::CompileError::from_parser(
                Some(*token),
                &format!("Expected {directive} content after '@{directive}'"),
            ));
        }

        Ok(result)
    }
}
