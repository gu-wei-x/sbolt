use crate::codegen::parser::tokenizer::{TokenStream, get_nth_token};
use crate::codegen::parser::types::optimizer::{self, Optimizer};
use crate::codegen::parser::{Token, tokenizer};
use crate::codegen::types::Block;
use crate::codegen::types::Span;
use crate::codegen::{CompilerOptions, consts};
use crate::types::template;
use crate::types::{error, result};
use winnow::stream::Stream as _;

#[allow(dead_code)]
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
pub(in crate::codegen) struct ParseContext<'a, 's> {
    block_kind: Kind,
    template_kind: template::Kind,
    tokens: Vec<Token>,
    compiler_option: &'a CompilerOptions,
    source: &'s str,
}

impl<'a, 's> ParseContext<'a, 's> {
    pub(in crate::codegen) fn new(
        block_kind: Kind,
        template_kind: template::Kind,
        option: &'a CompilerOptions,
        source: &'s str,
    ) -> Self {
        Self {
            block_kind: block_kind,
            tokens: Vec::new(),
            template_kind: template_kind,
            compiler_option: option,
            source: source,
        }
    }

    pub(in crate::codegen) fn clone_for(&self, block_kind: Kind) -> Self {
        Self {
            block_kind: block_kind,
            tokens: Vec::new(),
            template_kind: self.template_kind,
            compiler_option: self.compiler_option,
            source: self.source,
        }
    }

    pub(in crate::codegen) fn block_kind(&self) -> Kind {
        self.block_kind
    }

    pub(in crate::codegen) fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub(in crate::codegen) fn is_block(&self) -> bool {
        matches!(self.block_kind, Kind::KCONTENT | Kind::KROOT | Kind::KCODE)
    }

    pub(in crate::codegen) fn is_content(&self) -> bool {
        matches!(
            self.block_kind,
            Kind::KCONTENT | Kind::KROOT | Kind::KINLINEDCONTENT
        )
    }

    pub(in crate::codegen) fn is_code(&self) -> bool {
        matches!(
            self.block_kind,
            Kind::KCODE | Kind::KFUNCTIONS | Kind::KINLINEDCODE | Kind::KLAYOUT | Kind::KUSE
        )
    }

    pub(in crate::codegen) fn is_inline(&self) -> bool {
        matches!(self.block_kind, Kind::KINLINEDCODE | Kind::KINLINEDCONTENT)
    }

    pub(in crate::codegen) fn consume<'s1>(
        &mut self,
        source: &'s1 str,
    ) -> result::Result<Option<Block<'s1>>> {
        let optimizer = optimizer::HtmlOptimizer::new(self.compiler_option);
        let mut span = Span::new(source);
        for token in &self.tokens {
            if optimizer.accept(token) {
                span.push_token(*token);
            }
        }

        self.tokens.clear();
        // workaround fix later.
        let context = if matches!(self.block_kind, Kind::KROOT) {
            &self.clone_for(Kind::KCONTENT)
        } else {
            self
        };

        // could be empty block.
        match span.is_empty() {
            true => Ok(None),
            false => Ok(Some(Self::create_block(context, None, span)?)),
        }
    }

    pub(in crate::codegen) fn source(&self) -> &'s str {
        self.source
    }
}

impl<'a, 's> ParseContext<'a, 's> {
    pub(in crate::codegen) fn switch_if_possible(
        &self,
        token_stream: &mut TokenStream,
    ) -> result::Result<(bool, Self)> {
        let source = self.source;
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
            return Ok((false, self.clone_for(self.block_kind())));
        }

        let next_token = next_token.unwrap();
        match next_token.kind() {
            tokenizer::Kind::AT => {
                // @@ to escape @, don't switch context
                Ok((false, self.clone_for(self.block_kind())))
            }
            tokenizer::Kind::EXPRESSION => {
                let exp = &source[next_token.range()];
                match exp {
                    consts::DIRECTIVE_KEYWORD_USE => {
                        // block but not code kind.
                        if self.is_block() && !self.is_code() {
                            // switch to code context from root|content.
                            Ok((true, self.clone_for(Kind::KUSE)))
                        } else {
                            Err(error::CompileError::from_parser(
                                source,
                                Some(*next_token),
                                "The 'use' directive is only allowed in the block content context.",
                            ))
                        }
                    }
                    consts::DIRECTIVE_KEYWORD_LAYOUT => {
                        if self.block_kind() == Kind::KROOT {
                            // only allowed in root context.
                            Ok((true, self.clone_for(Kind::KLAYOUT)))
                        } else {
                            Err(error::CompileError::from_parser(
                                source,
                                Some(*next_token),
                                "The 'layout' directive is only allowed in the root context.",
                            ))
                        }
                    }
                    consts::KEYWORD_SECTION => {
                        if self.is_block() && self.block_kind() != Kind::KSECTION {
                            // todo: how to detect nested section in side section?
                            // do it before parsing end?
                            Ok((true, self.clone_for(Kind::KSECTION)))
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
                            Ok((true, self.clone_for(Kind::KINLINEDCONTENT)))
                        } else {
                            Ok((true, self.clone_for(Kind::KINLINEDCODE)))
                        }
                    }
                }
            }
            tokenizer::Kind::OPARENTHESIS => {
                // inlined
                if self.is_code() {
                    Ok((true, self.clone_for(Kind::KINLINEDCONTENT)))
                } else {
                    Ok((true, self.clone_for(Kind::KINLINEDCODE)))
                }
            }
            tokenizer::Kind::OCURLYBRACKET => {
                if self.is_code() {
                    Ok((true, self.clone_for(Kind::KCONTENT)))
                } else {
                    Ok((true, self.clone_for(Kind::KCODE)))
                }
            }
            tokenizer::Kind::ASTERISK => {
                if self.is_content() {
                    Ok((true, self.clone_for(Kind::KCOMMENT)))
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
                Ok((false, self.clone_for(self.block_kind())))
            }
        }
    }
}

impl<'a, 's> ParseContext<'a, 's> {
    pub(in crate::codegen) fn create_block<'s1>(
        context: &ParseContext,
        name: Option<String>,
        span: Span<'s1>,
    ) -> result::Result<Block<'s1>> {
        match name {
            Some(name) => Ok(Block::new_section(&name, span)),
            None => {
                // convert to block.
                match context.block_kind() {
                    Kind::KCODE => Ok(Block::new_code(span)),
                    Kind::KCOMMENT => Ok(Block::new_comment(span)),
                    Kind::KCONTENT => Ok(Block::new_content(span)),
                    Kind::KFUNCTIONS => Ok(Block::new_functions(span)),
                    Kind::KINLINEDCODE => Ok(Block::new_inline_code(span)),
                    Kind::KINLINEDCONTENT => Ok(Block::new_inline_content(span)),
                    Kind::KLAYOUT => Ok(Block::new_layout(span)),
                    Kind::KRENDER => Ok(Block::new_render(span)),
                    Kind::KROOT => Ok(Block::new_root(span)),
                    Kind::KSECTION => Ok(Block::new_section("", span)),
                    Kind::KUSE => Ok(Block::new_use(span)),
                }
            }
        }
    }
}
