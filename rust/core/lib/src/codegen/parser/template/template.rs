use crate::codegen::parser::template::block::Block;
use crate::codegen::parser::template::{Context, ParseContext};
use crate::codegen::parser::tokenizer::TokenStream;
use crate::codegen::parser::tokenizer::{self, Tokenizer};
use crate::types::{error, result};
use winnow::stream::TokenSlice;

pub(crate) struct Template<'a> {
    pub(crate) namespace: Option<String>,
    pub(crate) blocks: Vec<Block<'a>>,
}

impl<'a> Template<'a> {
    fn new(namespace: Option<String>, blocks: Vec<Block<'a>>) -> Self {
        Template { namespace, blocks }
    }
}

impl<'a> Template<'a> {
    pub(crate) fn from(source: &'a str, namespace: Option<String>) -> result::Result<Self> {
        let tokenizer = Tokenizer::new(source);
        let tokens = tokenizer.into_vec();
        let mut token_stream = TokenSlice::new(&tokens);
        let blocks = Block::parse_doc(source, &mut token_stream)?;
        let template = Template::new(namespace, blocks);
        Ok(template)
    }
}

impl<'a> Block<'a> {
    pub(crate) fn parse_doc(
        source: &'a str,
        token_stream: &mut TokenStream,
    ) -> result::Result<Vec<Block<'a>>> {
        let mut blocks = Vec::new();
        tokenizer::skip_whitespace_and_newline(token_stream);
        let mut context = ParseContext::new(Context::Content);
        let block = Block::parse(source, token_stream, &mut context)?;
        match block.blocks.is_empty() {
            true => {
                blocks.push(block);
            }
            false => {
                for block in block.blocks {
                    blocks.push(block);
                }
            }
        }

        match blocks.is_empty() {
            true => Err(error::Error::Parser(
                None,
                "Empty template is not allowed".to_string(),
            )),
            false => Ok(blocks),
        }
    }
}
