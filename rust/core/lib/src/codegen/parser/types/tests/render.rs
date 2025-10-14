#![cfg(test)]
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::types::Block;
use crate::types::result;
use winnow::stream::{Stream as _, TokenSlice};

#[test]
#[should_panic]
fn block_parse_render_empty_stream() {
    let source = r#""#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    token_stream.next_token();
    Block::parse_render(source, &mut token_stream).unwrap();
}

#[test]
#[should_panic]
fn block_parse_render_starts_without_keyword() {
    let source = r#"test"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    token_stream.next_token();
    Block::parse_render(source, &mut token_stream).unwrap();
}

#[test]
#[should_panic]
fn block_parse_render_without_closing() {
    let source = r#"@render("#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    token_stream.next_token();
    Block::parse_render(source, &mut token_stream).unwrap();
}

#[test]
#[should_panic]
fn block_parse_render_without_closing_2() {
    let source = r#"@render(test, false"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    token_stream.next_token();
    Block::parse_render(source, &mut token_stream).unwrap();
}

#[test]
#[should_panic]
fn block_parse_render_second_param_is_not_bool() {
    let source = r#"@render(test, hello)"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    token_stream.next_token();
    Block::parse_render(source, &mut token_stream).unwrap();
}

#[test]
#[should_panic]
fn block_parse_render_second_param_is_not_exp() {
    let source = r#"@render(test, ,)"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    token_stream.next_token();
    Block::parse_render(source, &mut token_stream).unwrap();
}

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
    assert_eq!(block.location().line, 0);

    let source = r#"@render"#;
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
    let root_span = match block {
        Block::KRENDER(span) => span,
        _ => panic!("Expected KRENDER block"),
    };
    assert_eq!(root_span.blocks().len(), 1);
    assert_eq!(root_span.blocks()[0].content(), "test");

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
    let root_span = match block {
        Block::KRENDER(span) => span,
        _ => panic!("Expected KRENDER block"),
    };
    assert_eq!(root_span.blocks().len(), 2);
    assert_eq!(root_span.blocks()[0].content(), "test");
    assert_eq!(root_span.blocks()[1].content(), "true");

    let source = r#"@render(test, false)"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    token_stream.next_token().unwrap();
    let block = Block::parse_render(source, &mut token_stream)?;
    assert!(matches!(block, Block::KRENDER(_)));
    let root_span = match block {
        Block::KRENDER(span) => span,
        _ => panic!("Expected KRENDER block"),
    };
    assert_eq!(root_span.blocks().len(), 2);
    assert_eq!(root_span.blocks()[0].content(), "test");
    assert_eq!(root_span.blocks()[1].content(), "false");

    Ok(())
}

#[test]
#[should_panic]
fn block_parse_render_more_than_2_params() {
    let source = r#"@render(test, true, test)"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    token_stream.next_token().unwrap();
    Block::parse_render(source, &mut token_stream).unwrap();
}
