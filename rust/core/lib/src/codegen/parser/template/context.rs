use crate::codegen::consts;
use crate::codegen::parser::Token;
use crate::codegen::parser::template::block::{self, Block};
use crate::codegen::parser::tokenizer::TokenStream;
use crate::codegen::parser::tokenizer::{self, get_nth_token};
use crate::types::{error, result};
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

        let mut result = Block::new(None, self.kind(), source);
        for token in &self.tokens {
            result.push_token(*token);
        }

        self.tokens.clear();
        Some(result)
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
                    return Err(error::CompileError::from_parser(
                        None,
                        "Expecting '@' token to start context extraction.",
                    ));
                }
            }
            _ => {
                return Err(error::CompileError::from_parser(
                    None,
                    "Empty token stream when expecting '@' token to start context extraction.",
                ));
            }
        };

        // check the next token after '@'
        let next_token = get_nth_token(token_stream, 1);
        if None == next_token {
            // no token after '@', don't switch context
            return Ok((false, ParseContext::new(self.kind())));
        }

        let next_token = next_token.unwrap();
        match next_token.kind() {
            tokenizer::Kind::AT => {
                // @@ to escape @, don't switch context
                Ok((false, ParseContext::new(self.kind())))
            }
            tokenizer::Kind::EXPRESSION => {
                let exp = &source[next_token.range()];
                match exp {
                    consts::DIRECTIVE_KEYWORD_USE => {
                        // block but not code kind.
                        if self.kind().is_block_kind() && !self.kind().is_code_kind() {
                            // switch to code context from root|content.
                            Ok((true, ParseContext::new(block::Kind::DIRECTIVE)))
                        } else {
                            Err(error::CompileError::from_parser(
                                Some(*next_token),
                                "The 'use' directive is only allowed in the block content context.",
                            ))
                        }
                    }
                    consts::DIRECTIVE_KEYWORD_LAYOUT => {
                        if self.kind() == block::Kind::ROOT {
                            // only allowed in root context.
                            Ok((true, ParseContext::new(block::Kind::DIRECTIVE)))
                        } else {
                            Err(error::CompileError::from_parser(
                                Some(*next_token),
                                "The 'layout' directive is only allowed in the root context.",
                            ))
                        }
                    }
                    consts::KEYWORD_SECTION => {
                        if self.kind().is_block_kind() && self.kind() != block::Kind::SECTION {
                            // todo: how to detect nested section in side section?
                            // do it before parsing end?
                            Ok((true, ParseContext::new(block::Kind::SECTION)))
                        } else {
                            Err(error::CompileError::from_parser(
                                Some(*next_token),
                                "The 'section' is only allowed in the block context.",
                            ))
                        }
                    }
                    _ => {
                        // inlined
                        if self.kind().is_code_kind() {
                            Ok((true, ParseContext::new(block::Kind::INLINEDCONTENT)))
                        } else {
                            Ok((true, ParseContext::new(block::Kind::INLINEDCODE)))
                        }
                    }
                }
            }
            tokenizer::Kind::OPARENTHESIS => {
                // inlined
                if self.kind().is_code_kind() {
                    Ok((true, ParseContext::new(block::Kind::INLINEDCONTENT)))
                } else {
                    Ok((true, ParseContext::new(block::Kind::INLINEDCODE)))
                }
            }
            tokenizer::Kind::OCURLYBRACKET => {
                if self.kind().is_code_kind() {
                    Ok((true, ParseContext::new(block::Kind::CONTENT)))
                } else {
                    Ok((true, ParseContext::new(block::Kind::CODE)))
                }
            }
            tokenizer::Kind::ASTERISK => {
                if self.kind().is_content_kind() {
                    Ok((true, ParseContext::new(block::Kind::COMMENT)))
                } else {
                    Err(error::CompileError::from_parser(
                        Some(*next_token),
                        "@* comments are only allowed in content context.",
                    ))
                }
            }
            _ => {
                // Don't switch context
                Ok((false, ParseContext::new(self.kind())))
            }
        }
    }
}
