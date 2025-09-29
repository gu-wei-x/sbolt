use crate::codegen::parser::Token;
use crate::codegen::parser::template::block::{self, Block};
use crate::codegen::parser::tokenizer;
use crate::codegen::parser::tokenizer::TokenStream;
use crate::codegen::{consts, parser};
use crate::types::{error, result};
use winnow::stream::Stream as _;

#[derive(PartialEq, Eq)]
pub(crate) enum Context {
    Content,
    Code,
}

pub(crate) struct ParseContext {
    kind: Context,
    tokens: Vec<Token>,
}

impl ParseContext {
    pub(crate) fn new(kind: Context) -> Self {
        Self {
            kind: kind,
            tokens: Vec::new(),
        }
    }

    pub(crate) fn is_content(&self) -> bool {
        self.kind == Context::Content
    }

    pub(crate) fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub(crate) fn should_switch(
        &self,
        source: &str,
        start_token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<bool> {
        if start_token.kind() != tokenizer::Kind::AT {
            return Err(error::Error::from_parser(
                Some(*start_token),
                "Expected '@' token to start context extraction.",
            ));
        }

        let offset = match token_stream.peek_token() {
            Some(token) => {
                if token.range() == start_token.range() {
                    1
                } else {
                    // already consumed '@' token
                    0
                }
            }
            _ => {
                return Ok(false);
            }
        };

        // check token after @, only swith context if legal,
        // else keeps current context unchanged to fail at compilation stage.
        match token_stream.offset_at(offset) {
            Ok(offset) => {
                if let Some(next_next_token) = token_stream.iter_offsets().nth(offset) {
                    match next_next_token.1.kind() {
                        tokenizer::Kind::EXPRESSION => {
                            let exp = &source[next_next_token.1.range()];
                            match exp {
                                consts::DIRECTIVE_KEYWORD_USE
                                | consts::DIRECTIVE_KEYWORD_LAYOUT => {
                                    return Ok(self.is_content());
                                }
                                consts::KEYWORD_SECTION => {
                                    // don't switch context
                                    return Ok(false);
                                }
                                _ => {
                                    // inlined
                                    return Ok(true);
                                }
                            }
                        }
                        tokenizer::Kind::OPARENTHESIS => {
                            // @(), inlined.
                            return Ok(true);
                        }
                        tokenizer::Kind::OCURLYBRACKET => {
                            // @{}, block.
                            return Ok(true);
                        }
                        _ => {
                            // don't switch context
                            return Ok(false);
                        }
                    }
                }
            }
            _ => {
                // don't switch context
                return Ok(false);
            }
        }

        // don't switch context
        Ok(false)
    }

    pub(crate) fn to_block<'a>(&mut self, source: &'a str) -> Option<Block<'a>> {
        // TODO: create block from current context and destruct current data.
        if self.tokens.is_empty() || self.tokens[0].kind() == tokenizer::Kind::EOF {
            return None;
        }

        let length = self.tokens.len();
        let start = self.tokens[0].range().start;
        let end = self.tokens[length - 1].range().end;
        let mut block = Block::default();
        match self.kind {
            Context::Content => {
                block.with_span(parser::Span {
                    kind: block::Kind::CONTENT(&source[start..end]),
                    start: start,
                    end: end,
                });
            }
            Context::Code => {
                block.with_span(parser::Span {
                    kind: block::Kind::CODE(&source[start..end]),
                    start: start,
                    end: end,
                });
            }
        }

        self.tokens.clear();
        Some(block)
    }
}
