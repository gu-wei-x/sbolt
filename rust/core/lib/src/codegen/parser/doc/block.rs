use crate::{
    codegen::{parser::tokenizer::TokenStream, types::Block},
    types::{error, result},
};

impl<'a> Block<'a> {
    pub(in crate::codegen) fn from(
        source: &'a str,
        _token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        Err(error::CompileError::from_parser(
            source,
            None,
            "Not implemented",
        ))
    }
}
