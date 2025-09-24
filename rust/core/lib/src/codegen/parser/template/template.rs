use crate::codegen::parser::template::block::Block;
use crate::codegen::parser::tokenizer::TokenStream;
use crate::codegen::parser::tokenizer::{self, Tokenizer};
use crate::types::{error, result};
use winnow::stream::Stream as _;
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
    pub(in crate::codegen::parser::template) fn parse_doc(
        source: &'a str,
        token_stream: &mut TokenStream,
    ) -> result::Result<Vec<Block<'a>>> {
        let mut blocks = Vec::new();
        tokenizer::skip_whitespace_and_newline(token_stream);
        while let Some(next_token) = token_stream.peek_token() {
            match next_token.kind() {
                tokenizer::Kind::EOF => break,
                tokenizer::Kind::NEWLINE => {
                    // consume newline.
                    token_stream.next_token();
                }
                tokenizer::Kind::AT => {
                    // TODO: doc level, use, section, layout, need to peek next-next to decide.
                    // keyword: use, section, layout
                    match token_stream.offset_at(1) {
                        Ok(1) => {
                            if let Some(next_next_token) = token_stream.iter_offsets().nth(1) {
                                println!("************************************");
                                println!("next next token: {:?}", next_next_token.1);
                                match next_next_token.1.kind() {
                                    tokenizer::Kind::EXPRESSION => {
                                        // get the value and compare with keywords.
                                        println!("************************************");
                                        println!("next next token: {:?}", next_next_token.1);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }

                    // consume @
                    let code_block = Block::parse_code(source, next_token, token_stream)?;
                    blocks.push(code_block);
                }
                _ => {
                    let content_block =
                        Block::parse_content(source, next_token, token_stream, false)?;
                    blocks.push(content_block);
                }
            }
        }

        match blocks.is_empty() {
            true => Err(error::Error::Parser(None, "Empty template is not allowed")),
            false => Ok(blocks),
        }
    }
}
