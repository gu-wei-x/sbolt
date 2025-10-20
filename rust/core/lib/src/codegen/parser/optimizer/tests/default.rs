#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::parser::optimizer::create_optimizer;
use crate::codegen::parser::tokenizer::{Kind, Tokenizer};

#[test]
fn accept_with_no_optimization() {
    let source = r#"<html>\n  </html>"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let options = CompilerOptions::default();
    let mut optimizer = create_optimizer(crate::types::template::Kind::KHTML, &options);
    for token in tokens {
        match token.kind() {
            Kind::EOF => assert!(!optimizer.accept(source, &token)),
            _ => assert!(optimizer.accept(source, &token)),
        }
    }
}
