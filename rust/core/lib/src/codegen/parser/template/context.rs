use crate::codegen::consts;
use crate::codegen::parser::Token;
use crate::codegen::parser::template::block::{self, Block};
use crate::codegen::parser::tokenizer;
use crate::codegen::parser::tokenizer::TokenStream;
use crate::types::{error, result};
use std::ops::Range;
use winnow::stream::Stream as _;

#[derive(Clone)]
pub(crate) struct ParseContext {
    kind: block::Kind,
    tokens: Vec<Token>,
}

impl ParseContext {
    pub(crate) fn new(kind: block::Kind) -> Self {
        Self {
            kind: kind,
            tokens: Vec::new(),
        }
    }

    pub(crate) fn kind(&self) -> block::Kind {
        self.kind
    }

    pub(crate) fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    #[allow(dead_code)]
    pub(crate) fn is_content(&self) -> bool {
        self.kind == block::Kind::CONTENT
            || self.kind == block::Kind::ROOT
            || self.kind == block::Kind::INLINEDCONTENT
    }

    #[allow(dead_code)]
    pub(crate) fn is_code(&self) -> bool {
        self.kind == block::Kind::CODE
            || self.kind == block::Kind::FUNCTIONS
            || self.kind == block::Kind::DIRECTIVE
            || self.kind == block::Kind::INLINEDCODE
    }

    pub(crate) fn consume<'a>(&mut self, source: &'a str) -> Option<Block<'a>> {
        // consume current tokens to create a block and destruct current data.
        if self.tokens.is_empty() || self.tokens[0].kind() == tokenizer::Kind::EOF {
            return None;
        }

        let length = self.tokens.len();
        let start = self.tokens[0].range().start;
        let end = self.tokens[length - 1].range().end;
        let content = &source[start..end];
        self.tokens.clear();
        Some(Block::new(
            None,
            Range {
                start: start,
                end: end,
            },
            self.kind(),
            content,
        ))
    }

    pub(crate) fn switch_if_possible(
        &self,
        source: &str,
        token_stream: &mut TokenStream,
    ) -> result::Result<(bool, Self)> {
        // first token must be '@'
        match token_stream.peek_token() {
            Some(token) => {
                if token.kind() != tokenizer::Kind::AT {
                    return Err(error::Error::from_parser(
                        None,
                        "Expecting '@' token to start context extraction.",
                    ));
                }
            }
            _ => {
                return Err(error::Error::from_parser(
                    None,
                    "Empty token stream when expecting '@' token to start context extraction.",
                ));
            }
        };

        // check the next token after '@'
        let offset = 1;
        match token_stream.offset_at(offset) {
            Ok(offset) => {
                if let Some(next_token) = token_stream.iter_offsets().nth(offset) {
                    match next_token.1.kind() {
                        tokenizer::Kind::AT => {
                            // @@ to escape @, don't switch context
                            return Ok((false, ParseContext::new(self.kind())));
                        }
                        tokenizer::Kind::EXPRESSION => {
                            let exp = &source[next_token.1.range()];
                            match exp {
                                consts::DIRECTIVE_KEYWORD_USE => {
                                    // block but not code kind.
                                    if self.kind().is_block_kind() && !self.kind().is_code_kind() {
                                        // switch to code context from root|content.
                                        return Ok((
                                            true,
                                            ParseContext::new(block::Kind::DIRECTIVE),
                                        ));
                                    } else {
                                        return Err(error::Error::from_parser(
                                            Some(*next_token.1),
                                            "The 'use' directive is only allowed in the block content context.",
                                        ));
                                    }
                                }
                                consts::DIRECTIVE_KEYWORD_LAYOUT => {
                                    if self.kind() == block::Kind::ROOT {
                                        // only allowed in root context.
                                        return Ok((
                                            true,
                                            ParseContext::new(block::Kind::DIRECTIVE),
                                        ));
                                    } else {
                                        return Err(error::Error::from_parser(
                                            Some(*next_token.1),
                                            "The 'layout' directive is only allowed in the root context.",
                                        ));
                                    }
                                }
                                consts::KEYWORD_SECTION => {
                                    if self.kind().is_block_kind()
                                        && self.kind() != block::Kind::SECTION
                                    {
                                        // todo: how to detect nested section in side section?
                                        // do it before parsing end?
                                        return Ok((true, ParseContext::new(block::Kind::SECTION)));
                                    } else {
                                        return Err(error::Error::from_parser(
                                            Some(*next_token.1),
                                            "The 'section' is only allowed in the block context.",
                                        ));
                                    }
                                }
                                _ => {
                                    // inlined
                                    if self.kind().is_code_kind() {
                                        return Ok((
                                            true,
                                            ParseContext::new(block::Kind::INLINEDCONTENT),
                                        ));
                                    } else {
                                        return Ok((
                                            true,
                                            ParseContext::new(block::Kind::INLINEDCODE),
                                        ));
                                    }
                                }
                            }
                        }
                        tokenizer::Kind::OPARENTHESIS => {
                            // inlined
                            if self.kind().is_code_kind() {
                                return Ok((true, ParseContext::new(block::Kind::INLINEDCONTENT)));
                            } else {
                                return Ok((true, ParseContext::new(block::Kind::INLINEDCODE)));
                            }
                        }
                        tokenizer::Kind::OCURLYBRACKET => {
                            if self.kind().is_code_kind() {
                                return Ok((true, ParseContext::new(block::Kind::CONTENT)));
                            } else {
                                return Ok((true, ParseContext::new(block::Kind::CODE)));
                            }
                        }
                        tokenizer::Kind::ASTERISK => {
                            if self.kind().is_content_kind() {
                                return Ok((true, ParseContext::new(block::Kind::COMMENT)));
                            } else {
                                return Err(error::Error::from_parser(
                                    Some(*next_token.1),
                                    "@* comments are only allowed in content context.",
                                ));
                            }
                        }
                        _ => {
                            // Don't switch context
                            return Ok((false, ParseContext::new(self.kind())));
                        }
                    }
                }
            }
            _ => {
                // token after '@' not found, don't switch context
                return Ok((false, ParseContext::new(self.kind())));
            }
        }

        // no token after '@', don't switch context
        Ok((false, ParseContext::new(self.kind())))
    }
}
