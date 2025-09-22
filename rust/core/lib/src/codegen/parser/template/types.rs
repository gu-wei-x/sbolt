use crate::codegen::parser;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum Kind<'a> {
    CODE(&'a str),
    CONTENT(&'a str),
    DOC(&'a str),
    // code with content or content with code.
    MIXED,
}

impl<'a> Default for Kind<'a> {
    fn default() -> Self {
        Self::MIXED
    }
}

#[derive(Debug)]
pub(crate) struct Block<'a> {
    pub(crate) span: parser::Span<Kind<'a>>,
    pub(crate) blocks: Vec<Block<'a>>,
}

impl<'a> Default for Block<'a> {
    fn default() -> Self {
        Self {
            span: parser::Span::<Kind<'a>>::default(),
            blocks: vec![],
        }
    }
}

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::template) fn with_span(
        &mut self,
        span: parser::Span<Kind<'a>>,
    ) -> &mut Self {
        self.span = span;
        self
    }

    pub(in crate::codegen::parser::template) fn push_block(
        &mut self,
        block: Block<'a>,
    ) -> &mut Self {
        self.blocks.push(block);
        self
    }
}
