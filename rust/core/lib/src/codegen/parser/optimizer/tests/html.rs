#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::parser::Token;
use crate::codegen::parser::optimizer::{Optimizer, html};
use crate::codegen::parser::tokenizer::{Kind, Tokenizer};

#[test]
fn accept_with_no_optimization() {
    let source = r#"<html>\n  </html>"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let options = CompilerOptions::default();
    let mut optimizer = html::HtmlOptimizer::new(&options);
    for token in tokens {
        match token.kind() {
            Kind::EOF => assert!(!optimizer.accept(&token)),
            _ => assert!(optimizer.accept(&token)),
        }
    }
}

#[test]
fn accept_with_optimization_ignore_line_feed() {
    let source = "<html>\n\r\r\n";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let options = CompilerOptions::default().with_optimization(true);
    let mut optimizer = html::HtmlOptimizer::new(&options);
    let tokens = tokens
        .iter()
        .filter(|t| optimizer.accept(t))
        .collect::<Vec<&Token>>();

    let tokenizer = Tokenizer::new("<html>");
    let expected_tokens = tokenizer.into_vec();
    let expected = expected_tokens
        .iter()
        .filter(|t| t.kind() != Kind::EOF)
        .collect::<Vec<&Token>>();

    println!("{tokens:#?}");
    // todo: method to verify the type, length but not location info.
    assert_eq!(tokens, expected);
}
