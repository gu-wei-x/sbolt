#![cfg(test)]
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::parser::types::util;
use winnow::stream::TokenSlice;

#[test]
fn is_token_escaped() {
    let source = r#"@@test"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    assert!(util::is_token_escaped(&mut token_stream));

    let source = r#"@test"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    assert!(!util::is_token_escaped(&mut token_stream));
}
