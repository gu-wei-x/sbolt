#![cfg(test)]
use crate::codegen::parser::types::context::{self, ParseContext};
use crate::codegen::types::Block;
use crate::codegen::{CompilerOptions, parser::tokenizer::Tokenizer};
use crate::types::{result, template};
use winnow::stream::{Stream as _, TokenSlice};

// comments.
#[test]
fn block_parse_comment() -> result::Result<()> {
    let source = r#"@****test****@"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let token = token_stream.next_token().unwrap();
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(
        context::Kind::KCOMMENT,
        template::Kind::KHTML,
        &options,
        source,
    );
    let block = Block::parse_comment(token, &mut token_stream, &mut context)?;
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
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(
        context::Kind::KCOMMENT,
        template::Kind::KHTML,
        &options,
        source,
    );
    Block::parse_comment(token, &mut token_stream, &mut context).unwrap();
}

#[test]
#[should_panic]
fn block_parse_comment_invalid() {
    let source = r#"@test****"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let token = token_stream.next_token().unwrap();
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(
        context::Kind::KCOMMENT,
        template::Kind::KHTML,
        &options,
        source,
    );
    Block::parse_comment(token, &mut token_stream, &mut context).unwrap();
}

#[test]
#[should_panic]
fn block_parse_comment_invalid2() {
    let source = r#"@*test*"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    // ignore the eof.
    let mut token_stream = TokenSlice::new(&tokens[0..=3]);
    let token = token_stream.next_token().unwrap();
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(
        context::Kind::KCOMMENT,
        template::Kind::KHTML,
        &options,
        source,
    );
    Block::parse_comment(token, &mut token_stream, &mut context).unwrap();
}
