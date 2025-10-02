use crate::codegen::parser::template::{ParseContext, util};
use crate::codegen::parser::tokenizer::TokenStream;
use crate::{
    codegen::parser::tokenizer,
    types::{error, result},
};
use std::fmt::Debug;
use std::ops::Range;
use winnow::stream::Stream;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum Kind {
    CODE,
    COMMENT,
    INLINEDCODE,
    CONTENT,
    // layout, use.
    DIRECTIVE,
    INLINEDCONTENT,
    FUNCTIONS,
    SECTION,
    ROOT,
    // intermediate kind.
    UNKNOWN,
}

#[allow(dead_code)]
impl Kind {
    pub(crate) fn is_block_kind(&self) -> bool {
        matches!(
            self,
            Kind::CODE | Kind::CONTENT | Kind::ROOT | Kind::FUNCTIONS
        )
    }

    pub(crate) fn is_inlined_kind(&self) -> bool {
        matches!(self, Kind::INLINEDCODE | Kind::INLINEDCONTENT)
    }

    pub(crate) fn is_code_kind(&self) -> bool {
        matches!(self, Kind::CODE | Kind::FUNCTIONS | Kind::INLINEDCODE)
    }

    pub(crate) fn is_content_kind(&self) -> bool {
        matches!(self, Kind::CONTENT | Kind::INLINEDCONTENT | Kind::ROOT)
    }

    pub(crate) fn is_directive_kind(&self) -> bool {
        matches!(self, Kind::DIRECTIVE)
    }
}

#[derive(Clone)]
pub(crate) struct Block<'a> {
    // block like use, section could have name.
    name: Option<String>,
    kind: Kind,
    blocks: Vec<Block<'a>>,

    // block content will be generated from source with tokens.
    source: &'a str,
    span: Range<usize>,
    tokens: Vec<tokenizer::Token>,
}

impl<'a> Default for Block<'a> {
    fn default() -> Self {
        Self {
            name: None,
            kind: Kind::UNKNOWN,
            blocks: vec![],
            source: "",
            span: Range::<usize>::default(),
            tokens: vec![],
        }
    }
}

impl Debug for Block<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Block");
        debug_struct
            .field("name", &self.name)
            .field("kind", &self.kind)
            .field("blocks", &self.blocks)
            .field("span", &self.span)
            .field("content", &self.content());
        if matches!(self.kind(), Kind::UNKNOWN | Kind::ROOT) {
            debug_struct.field("source", &self.source);
        }
        debug_struct.finish()
    }
}

impl<'a> Block<'a> {
    pub(crate) fn new(name: Option<String>, kind: Kind, source: &'a str) -> Self {
        Self {
            name,
            kind,
            blocks: vec![],
            source,
            span: Range::<usize>::default(),
            tokens: vec![],
        }
    }

    pub(crate) fn kind(&self) -> Kind {
        self.kind
    }

    // TODO: implement later, might be use option to ignore some content.
    // like: optimzed to ignore newline, comments.
    // html, convert comments to html comments...
    pub(crate) fn content(&self) -> String {
        let mut content = String::new();
        for token in &self.tokens {
            content.push_str(&self.source[token.range()]);
        }
        content
    }

    pub(crate) fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub(crate) fn blocks(&self) -> &Vec<Block<'a>> {
        &self.blocks
    }

    pub(crate) fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    pub(crate) fn has_blocks(&self) -> bool {
        !self.blocks.is_empty()
    }

    pub(crate) fn with_kind(&mut self, kind: Kind) -> &mut Self {
        self.kind = kind;
        self
    }

    pub(crate) fn with_name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    // TODO: push token & push block should not call on the same object.
    pub(crate) fn push_block(&mut self, block: Block<'a>) -> &mut Self {
        // update container kind with first block.
        if matches!(self.kind(), Kind::UNKNOWN) {
            match block.kind() {
                Kind::CODE => {
                    self.kind = Kind::CODE;
                }
                Kind::CONTENT => {
                    self.kind = Kind::CONTENT;
                }
                _ => { /* no-op */ }
            }
            self.span.start = block.span.start;
            self.span.end = block.span.end;
        } else {
            self.span.end = block.span.end;
        }

        self.blocks.push(block);
        self
    }

    pub(crate) fn push_token(&mut self, token: tokenizer::Token) -> &mut Self {
        if self.tokens.is_empty() {
            self.span = token.range();
        } else {
            self.span.end = token.range().end;
        }
        self.tokens.push(token);
        self
    }
}

impl<'a> Block<'a> {
    pub(crate) fn parse_block_within_kinds(
        source: &'a str,
        open_kind: tokenizer::Kind,
        close_kind: tokenizer::Kind,
        token_stream: &mut TokenStream,
        context: &mut ParseContext,
    ) -> result::Result<Block<'a>> {
        // validate, first token must be open_kind and consume the token.
        let previous_token = token_stream.previous_tokens().last().copied();
        match token_stream.peek_token() {
            Some(token) => {
                if token.kind() != open_kind {
                    return Err(error::CompileError::from_parser(
                        previous_token,
                        "Expected opening delimiter",
                    ));
                } else {
                    token_stream.next_token();
                }
            }
            _ => {
                return Err(error::CompileError::from_parser(
                    previous_token,
                    "Expected opening delimiter",
                ));
            }
        }

        let mut depth = 1;
        let mut result = Block::default();
        while let Some(token) = token_stream.peek_token() {
            match token.kind() {
                k if k == open_kind => {
                    context.push(*token);
                    token_stream.next_token();
                    depth += 1;
                }
                k if k == close_kind => {
                    // should not push the closing delimiter to context.
                    // ignore the closing delimiter.
                    token_stream.next_token();
                    depth -= 1;
                    if depth == 0 {
                        break;
                    } else {
                        context.push(*token);
                    }
                }
                tokenizer::Kind::AT => {
                    if context.kind().is_inlined_kind() {
                        return Err(error::CompileError::from_parser(
                            Some(*token),
                            "Inlined block is not allowed to use '@' token",
                        ));
                    }
                    match context.switch_if_possible(source, token_stream) {
                        Ok((true, mut new_context)) => {
                            // 1. consume the current pending tokens belong to current context.
                            match context.consume(source) {
                                Some(block) => {
                                    result.push_block(block);
                                }
                                _ => {
                                    // no-op: as there is no pending tokens belong to current context.
                                }
                            }

                            // 2. switch context.
                            let block =
                                Block::parse_at_block(source, token_stream, &mut new_context)?;
                            result.push_block(block);
                        }
                        Ok((false, _)) => {
                            if util::is_token_escaped(token_stream) {
                                // consume 2 @ as one, only push one @ to context.
                                token_stream.next_token();
                            }
                            token_stream.next_token();
                            context.push(*token);
                        }
                        Err(e) => return Err(e),
                    }
                }
                _ => {
                    // consume and push to current context.
                    // TODO: for prettry, ignore the newline when generate code.
                    token_stream.next_token();
                    context.push(*token);
                }
            }
        }

        // not balanced.
        if depth != 0 {
            return Err(error::CompileError::from_parser(
                previous_token,
                "Unbalanced delimiters in block",
            ));
        }

        // consume the context.
        if let Some(block) = context.consume(source) {
            result.push_block(block);
        }

        match result.blocks.len() {
            0 => Err(error::CompileError::from_parser(
                previous_token,
                "Failed to parser block",
            )),
            1 => Ok(result.blocks.pop().unwrap()),
            _ => Ok(result),
        }
    }

    pub(crate) fn parse(
        source: &'a str,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        // skip leading whitespace and linefeeds.
        tokenizer::skip_whitespace_and_newline(token_stream);
        match token_stream.peek_token() {
            None => {
                return Err(error::CompileError::from_parser(None, "Empty stream"));
            }
            Some(_) => {
                let mut context = ParseContext::new(Kind::ROOT);
                let mut result = Block::new(None, Kind::ROOT, source);
                while let Some(token) = token_stream.peek_token() {
                    match token.kind() {
                        tokenizer::Kind::EOF => break,
                        tokenizer::Kind::NEWLINE => {
                            token_stream.next_token();
                            //for prettry, ignore the newline
                            context.push(*token);
                        }
                        tokenizer::Kind::AT => {
                            match context.switch_if_possible(source, token_stream) {
                                Ok((true, mut new_context)) => {
                                    // 1. consume the current pending tokens belong to current context.
                                    match context.consume(source) {
                                        Some(mut block) => {
                                            // workaround fix later.
                                            if block.kind() == Kind::ROOT {
                                                block.with_kind(Kind::CONTENT);
                                            }
                                            result.push_block(block);
                                        }
                                        _ => {
                                            // no-op: as there is no pending tokens belong to current context.
                                        }
                                    }

                                    // 2. switch context.
                                    let block = Block::parse_at_block(
                                        source,
                                        token_stream,
                                        &mut new_context,
                                    )?;
                                    result.push_block(block);
                                }
                                Ok((false, _)) => {
                                    if util::is_token_escaped(token_stream) {
                                        // consume 2 @ as one, only push one @ to context.
                                        token_stream.next_token();
                                    }
                                    token_stream.next_token();
                                    context.push(*token);
                                }
                                Err(e) => return Err(e),
                            }
                        }
                        _ => {
                            // consume and push to current context.
                            // TODO: for prettry, ignore the newline when generate code.
                            token_stream.next_token();
                            context.push(*token);
                        }
                    }
                }

                // consume the context.
                // TODO: for prettry, ignore the newline when generate code.
                match context.consume(source) {
                    Some(mut block) => {
                        // workaround fix later.
                        if block.kind() == Kind::ROOT {
                            block.with_kind(Kind::CONTENT);
                        }

                        result.push_block(block);
                    }
                    _ => { /* no-ops*/ }
                }

                match result.has_blocks() {
                    false => Err(error::CompileError::from_parser(None, "Empty block")),
                    true => Ok(result),
                }
            }
        }
    }
}
