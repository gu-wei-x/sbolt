use crate::codegen::parser::template::block::Block;
use crate::codegen::parser::tokenizer::{self, Tokenizer};
use crate::types::result;
use winnow::stream::TokenSlice;

pub(crate) struct Template<'a> {
    namespace: Option<String>,
    block: Block<'a>,
}

impl<'a> Template<'a> {
    fn new(namespace: Option<String>, block: Block<'a>) -> Self {
        Template { namespace, block }
    }
}

impl<'a> Template<'a> {
    pub(crate) fn from(source: &'a str, namespace: Option<String>) -> result::Result<Self> {
        let tokenizer = Tokenizer::new(source);
        let tokens = tokenizer.into_vec();
        let mut token_stream = TokenSlice::new(&tokens);

        // skip leading whitespace and newlines.
        tokenizer::skip_whitespace_and_newline(&mut token_stream);
        let block = Block::parse(source, &mut token_stream)?;
        let template = Template::new(namespace, block);
        Ok(template)
    }

    pub(crate) fn namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }

    pub(crate) fn block(&self) -> &Block<'a> {
        &self.block
    }
}
