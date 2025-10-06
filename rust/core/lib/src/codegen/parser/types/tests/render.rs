#![cfg(test)]
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::types::Block;
use crate::types::result;
use winnow::stream::{Stream as _, TokenSlice};

#[test]
fn block_parse_render_no_params() -> result::Result<()> {
    let source = r#"@render()"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    token_stream.next_token().unwrap();
    let block = Block::parse_render(source, &mut token_stream)?;
    assert!(matches!(block, Block::KRENDER(_)));
    assert_eq!(block.content(), "");
    Ok(())
}

#[test]
fn block_parse_render_single_param() -> result::Result<()> {
    let source = r#"@render(test)"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    token_stream.next_token().unwrap();
    let block = Block::parse_render(source, &mut token_stream)?;
    assert!(matches!(block, Block::KRENDER(_)));
    Ok(())
}

#[test]
fn block_parse_render_2_params() -> result::Result<()> {
    let source = r#"@render(test, true)"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    token_stream.next_token().unwrap();
    let block = Block::parse_render(source, &mut token_stream)?;
    assert!(matches!(block, Block::KRENDER(_)));
    Ok(())
}

#[test]
fn block_parse_render_more_than_2_params() -> result::Result<()> {
    let source = r#"@render(test, true, test)"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    token_stream.next_token().unwrap();
    let block = Block::parse_render(source, &mut token_stream)?;
    assert!(matches!(block, Block::KRENDER(_)));
    Ok(())
}
