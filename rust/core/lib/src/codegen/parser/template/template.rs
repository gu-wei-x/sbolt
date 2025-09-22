use crate::codegen::parser;
use crate::codegen::parser::template::Kind;
use crate::codegen::parser::template::types;
use crate::codegen::parser::template::types::Block;
use crate::codegen::parser::tokenizer::TokenStream;
use crate::codegen::parser::tokenizer::{self, Tokenizer};
use crate::types::result;
use winnow::stream::Stream as _;
use winnow::stream::TokenSlice;

pub(crate) struct Template<'a> {
    pub(crate) namespace: Option<String>,
    pub(crate) block: types::Block<'a>,
}

impl<'a> Template<'a> {
    fn new(namespace: Option<String>, block: types::Block<'a>) -> Self {
        Template {
            namespace,
            block: block,
        }
    }
}

impl<'a> Template<'a> {
    pub(crate) fn from(source: &'a str, namespace: Option<String>) -> result::Result<Self> {
        let tokenizer = Tokenizer::new(source);
        let tokens = tokenizer.into_vec();
        let mut token_stream = TokenSlice::new(&tokens);

        let block = types::Block::parse_doc(source, &mut token_stream)?;
        let template = Template::new(namespace, block);
        Ok(template)
    }
}

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::template) fn parse_doc(
        source: &'a str,
        token_stream: &mut TokenStream,
    ) -> result::Result<Block<'a>> {
        let mut block = Block::default();
        block.with_span(parser::Span {
            kind: Kind::DOC(&source[0..source.len()]),
            start: 0,
            end: source.len(),
        });

        tokenizer::skip_whitespace_and_newline(token_stream);
        while let Some(next_token) = token_stream.peek_token() {
            match next_token.kind() {
                tokenizer::Kind::EOF => break,
                tokenizer::Kind::NEWLINE => {
                    // consume newline.
                    token_stream.next_token();
                }
                tokenizer::Kind::AT => {
                    // consume @
                    let code_block = types::Block::parse_code(source, next_token, token_stream)?;
                    block.push_block(code_block);
                }
                _ => {
                    let content_block =
                        types::Block::parse_content(source, next_token, token_stream)?;
                    block.push_block(content_block);
                }
            }
        }

        Ok(block)
    }
}
