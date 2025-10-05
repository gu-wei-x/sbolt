#![allow(dead_code)]
use crate::codegen::parser::Token;
use crate::codegen::types::Block;
use crate::types::Location;
use std::ops::Range;

#[derive(Clone)]
pub(in crate::codegen) struct Span<'a> {
    blocks: Vec<Block<'a>>,
    location: Location,
    range: Range<usize>,
    tokens: Vec<Token>,
    source: &'a str,
}

impl<'a> Span<'a> {
    pub(in crate::codegen) fn new(source: &'a str) -> Self {
        Self {
            blocks: vec![],
            location: Location::default(),
            range: Range::<usize>::default(),
            tokens: vec![],
            source: source,
        }
    }

    pub(in crate::codegen) fn blocks(&self) -> &Vec<Block<'a>> {
        &self.blocks
    }

    pub(in crate::codegen) fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    pub(in crate::codegen) fn location(&self) -> Location {
        if self.has_blocks() {
            self.blocks
                .first()
                .map_or(Location::default(), |b| b.location())
        } else {
            self.tokens
                .first()
                .map_or(Location::default(), |t| t.location())
        }
    }

    pub(in crate::codegen) fn has_blocks(&self) -> bool {
        !self.blocks.is_empty()
    }

    pub(in crate::codegen) fn push_block(&mut self, block: Block<'a>) -> &mut Self {
        self.blocks.push(block);
        self
    }

    pub(in crate::codegen) fn push_token(&mut self, token: Token) -> &mut Self {
        self.tokens.push(token);
        self
    }
}
