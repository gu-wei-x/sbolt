#![cfg(test)]
use crate::codegen::consts;
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::types::Block;
use crate::types::result;
use quote::quote;
use winnow::stream::TokenSlice;

#[test]
fn generate_use_code() -> result::Result<()> {
    let source = "use test";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = Block::parse_directive(source, consts::DIRECTIVE_KEYWORD_USE, &mut token_stream)?;
    let code = &block.generate_code()?[0];
    let expected = quote! {
       use test;
    };
    assert_eq!(code.to_string(), expected.to_string());
    Ok(())
}
