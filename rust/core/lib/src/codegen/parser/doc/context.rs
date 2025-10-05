#![allow(dead_code)]
use winnow::stream::Stream as _;

use crate::codegen::consts;
use crate::codegen::parser::tokenizer::{TokenStream, get_nth_token};
use crate::codegen::parser::{Token, tokenizer};
use crate::codegen::types::Block;
use crate::codegen::types::Span;
use crate::types::{error, result};

#[derive(Clone, Copy, PartialEq)]
pub(in crate::codegen) enum Kind {
    KCODE,
    KCOMMENT,
    KCONTENT,
    KFUNCTIONS,
    KINLINEDCODE,
    KINLINEDCONTENT,
    KLAYOUT,
    KRENDER,
    KROOT,
    KSECTION,
    KUSE,
}

#[derive(Clone)]
pub(in crate::codegen) struct ParseContext {
    kind: Kind,
    tokens: Vec<Token>,
}

impl ParseContext {
    pub(in crate::codegen) fn new(kind: Kind) -> Self {
        Self {
            kind: kind,
            tokens: Vec::new(),
        }
    }

    pub(in crate::codegen) fn kind(&self) -> Kind {
        self.kind
    }

    pub(in crate::codegen) fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub(in crate::codegen) fn is_block(&self) -> bool {
        matches!(self.kind, Kind::KCONTENT | Kind::KROOT | Kind::KCODE)
    }

    pub(in crate::codegen) fn is_content(&self) -> bool {
        matches!(
            self.kind,
            Kind::KCONTENT | Kind::KROOT | Kind::KINLINEDCONTENT
        )
    }

    pub(in crate::codegen) fn is_code(&self) -> bool {
        matches!(
            self.kind,
            Kind::KCODE | Kind::KFUNCTIONS | Kind::KINLINEDCODE | Kind::KLAYOUT | Kind::KUSE
        )
    }

    pub(in crate::codegen) fn consume<'a>(
        &mut self,
        source: &'a str,
    ) -> result::Result<Option<Block<'a>>> {
        // consume current tokens to create a block and destruct current data.
        if self.tokens.is_empty() || self.tokens[0].kind() == tokenizer::Kind::EOF {
            return Ok(None);
        }

        let mut span = Span::new(source);
        for token in &self.tokens {
            span.push_token(*token);
        }

        self.tokens.clear();

        // convert to block.
        Ok(Some(Self::create_block(self, None, span)?))
    }
}

impl ParseContext {
    pub(in crate::codegen) fn switch_if_possible(
        &self,
        source: &str,
        token_stream: &mut TokenStream,
    ) -> result::Result<(bool, Self)> {
        // first token must be '@'
        match token_stream.peek_token() {
            Some(token) => {
                if token.kind() != tokenizer::Kind::AT {
                    return Err(error::CompileError::from_parser(
                        source,
                        None,
                        "Expecting '@' token to start context extraction.",
                    ));
                }
            }
            _ => {
                return Err(error::CompileError::from_parser(
                    source,
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
                        if self.is_block() && !self.is_code() {
                            // switch to code context from root|content.
                            Ok((true, ParseContext::new(Kind::KUSE)))
                        } else {
                            Err(error::CompileError::from_parser(
                                source,
                                Some(*next_token),
                                "The 'use' directive is only allowed in the block content context.",
                            ))
                        }
                    }
                    consts::DIRECTIVE_KEYWORD_LAYOUT => {
                        if self.kind() == Kind::KROOT {
                            // only allowed in root context.
                            Ok((true, ParseContext::new(Kind::KLAYOUT)))
                        } else {
                            Err(error::CompileError::from_parser(
                                source,
                                Some(*next_token),
                                "The 'layout' directive is only allowed in the root context.",
                            ))
                        }
                    }
                    consts::KEYWORD_SECTION => {
                        if self.is_block() && self.kind() != Kind::KSECTION {
                            // todo: how to detect nested section in side section?
                            // do it before parsing end?
                            Ok((true, ParseContext::new(Kind::KSECTION)))
                        } else {
                            Err(error::CompileError::from_parser(
                                source,
                                Some(*next_token),
                                "The 'section' is only allowed in the block context.",
                            ))
                        }
                    }
                    _ => {
                        // inlined
                        if self.is_code() {
                            Ok((true, ParseContext::new(Kind::KINLINEDCONTENT)))
                        } else {
                            Ok((true, ParseContext::new(Kind::KINLINEDCODE)))
                        }
                    }
                }
            }
            tokenizer::Kind::OPARENTHESIS => {
                // inlined
                if self.is_code() {
                    Ok((true, ParseContext::new(Kind::KINLINEDCONTENT)))
                } else {
                    Ok((true, ParseContext::new(Kind::KINLINEDCODE)))
                }
            }
            tokenizer::Kind::OCURLYBRACKET => {
                if self.is_code() {
                    Ok((true, ParseContext::new(Kind::KCONTENT)))
                } else {
                    Ok((true, ParseContext::new(Kind::KCODE)))
                }
            }
            tokenizer::Kind::ASTERISK => {
                if self.is_content() {
                    Ok((true, ParseContext::new(Kind::KCOMMENT)))
                } else {
                    Err(error::CompileError::from_parser(
                        source,
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

impl ParseContext {
    pub(in crate::codegen) fn create_block<'a>(
        context: &ParseContext,
        name: Option<String>,
        span: Span<'a>,
    ) -> result::Result<Block<'a>> {
        match name {
            Some(name) => Ok(Block::new_section(&name, span)),
            None => {
                // convert to block.
                match context.kind() {
                    Kind::KCODE => Ok(Block::new_code(span)),
                    Kind::KCOMMENT => Ok(Block::new_comment(span)),
                    Kind::KCONTENT => Ok(Block::new_content(span)),
                    Kind::KFUNCTIONS => Ok(Block::new_functions(span)),
                    Kind::KINLINEDCODE => Ok(Block::new_inline_code(span)),
                    Kind::KINLINEDCONTENT => Ok(Block::new_inline_content(span)),
                    Kind::KLAYOUT => Ok(Block::new_layout(span)),
                    Kind::KRENDER => Ok(Block::new_render(span)),
                    Kind::KROOT => Ok(Block::new_root(span)),
                    Kind::KSECTION => Err(error::CompileError::from_parser(
                        "",
                        None,
                        "Wrong type for crating block",
                    )),
                    Kind::KUSE => Ok(Block::new_use(span)),
                }
            }
        }
    }
}
