#![cfg(test)]
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::types::Block;
use crate::types::result;
use winnow::stream::{Stream as _, TokenSlice};

// comments.
#[test]
fn block_parse_comment() -> result::Result<()> {
    let source = r#"@****test****@"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let token = token_stream.next_token().unwrap();
    let block = Block::parse_comment(source, token, &mut token_stream)?;
    assert!(matches!(block, Block::KCOMMENT(_)));
    assert_eq!(block.content(), source);
    assert_eq!(block.location().line, 0);
    Ok(())
}

#[test]
#[should_panic]
fn block_parse_comment_without_closing() {
    let source = r#"@****test****"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let token = token_stream.next_token().unwrap();
    Block::parse_comment(source, token, &mut token_stream).unwrap();
}
