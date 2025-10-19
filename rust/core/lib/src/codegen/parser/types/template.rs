use crate::codegen::parser::types::context;
use crate::types::template;
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
        kind: template::Kind,
        compiler_options: &crate::codegen::compiler::CompilerOptions,
    ) -> result::Result<Self> {
        let tokenizer = Tokenizer::new(source);
        let tokens = tokenizer.into_vec();
        let mut token_stream = TokenSlice::new(&tokens);
        let mut context =
            context::ParseContext::new(context::Kind::KROOT, kind, compiler_options, source);
        // skip leading whitespace and newlines.
        tokenizer::skip_whitespace_and_newline(&mut token_stream);
        let block = Block::parse(&mut token_stream, &mut context)?;
        let template = Template::new(namespace, block, kind);
        Ok(template)
    }
}
