#![allow(dead_code)]
use crate::{
    codegen::{
        parser::tokenizer::{self, Tokenizer},
        types::{Block, Template},
    },
    types::result,
};
use winnow::stream::TokenSlice;

impl<'a> Template<'a> {
    pub(in crate::codegen) fn from(
        source: &'a str,
        namespace: Option<String>,
    ) -> result::Result<Self> {
        let tokenizer = Tokenizer::new(source);
        let tokens = tokenizer.into_vec();
        let mut token_stream = TokenSlice::new(&tokens);

        // skip leading whitespace and newlines.
        tokenizer::skip_whitespace_and_newline(&mut token_stream);
        let block = Block::from(source, &mut token_stream)?;
        let template = Template::new(namespace, block);
        Ok(template)
    }
}
