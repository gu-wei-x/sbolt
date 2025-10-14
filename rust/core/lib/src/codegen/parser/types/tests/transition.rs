#![cfg(test)]
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::parser::types::context::{Kind, ParseContext};
use crate::codegen::types::Block;
use crate::types::result;
use winnow::stream::{Stream as _, TokenSlice};

#[test]
fn parse_transition_block() -> result::Result<()> {
    let source = r#"@test"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = Block::parse_transition_block(
        source,
        &mut token_stream,
        &mut ParseContext::new(Kind::KCODE),
    )?;
    assert!(matches!(block, Block::KCODE(_)));
    Ok(())
}

#[test]
#[should_panic]
fn parse_transition_block_starts_without_transition_symbol() {
    let source = r#"@test"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    // change the first token to not be '@'
    token_stream.next_token();
    Block::parse_transition_block(
        source,
        &mut token_stream,
        &mut ParseContext::new(Kind::KCODE),
    )
    .unwrap();
}

#[test]
#[should_panic]
fn parse_transition_block_empty_stream() {
    let source = r#""#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    // change the first token to not be '@'
    token_stream.next_token();
    Block::parse_transition_block(
        source,
        &mut token_stream,
        &mut ParseContext::new(Kind::KCODE),
    )
    .unwrap();
}

#[test]
#[should_panic]
fn parse_transition_block_single_transition_symbol() {
    let source = r#"@"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens[0..1]);
    Block::parse_transition_block(
        source,
        &mut token_stream,
        &mut ParseContext::new(Kind::KCODE),
    )
    .unwrap();
}

#[test]
#[should_panic]
fn parse_transition_block_invalid_token() {
    let source = r#"@!"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    Block::parse_transition_block(
        source,
        &mut token_stream,
        &mut ParseContext::new(Kind::KCODE),
    )
    .unwrap();
}

#[test]
fn parse_transition_block_render() -> result::Result<()> {
    let source = r#"@render(test)"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = Block::parse_transition_block(
        source,
        &mut token_stream,
        &mut ParseContext::new(Kind::KCODE),
    )?;
    assert!(matches!(block, Block::KRENDER(_)));
    Ok(())
}

#[test]
#[should_panic]
fn parse_transition_block_render_invalid_context() {
    let source = r#"@render(test)"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    Block::parse_transition_block(
        source,
        &mut token_stream,
        &mut ParseContext::new(Kind::KCONTENT),
    )
    .unwrap();
}
