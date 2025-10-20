#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::parser::Token;
use crate::codegen::parser::optimizer::{Optimizer, html};
use crate::codegen::parser::tokenizer::{Kind, Tokenizer};

fn tokens_to_string<'s>(source: &'s str, tokens: &[Token]) -> String {
    let mut content = String::new();
    for token in tokens {
        content.push_str(&source[token.range()]);
    }
    content
}

#[test]
fn accept_with_no_optimization() {
    let source = r#"<html>\n  </html>"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let options = CompilerOptions::default();
    let mut optimizer = html::HtmlOptimizer::new(&options);
    for token in tokens {
        match token.kind() {
            Kind::EOF => assert!(!optimizer.accept(source, &token)),
            _ => assert!(optimizer.accept(source, &token)),
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
        .into_iter()
        .filter(|t| optimizer.accept(source, t))
        .collect::<Vec<Token>>();

    let content = tokens_to_string(source, &tokens);
    let expected = "<html>";
    assert_eq!(content, expected);
}

#[test]
fn accept_with_optimization_ignore_spaces_and_lf() {
    let source = r#"
        < div class = "c1 c2" >  
            test
        < /div >
    "#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let options = CompilerOptions::default().with_optimization(true);
    let mut optimizer = html::HtmlOptimizer::new(&options);
    let tokens = tokens
        .into_iter()
        .filter(|t| optimizer.accept(source, t))
        .collect::<Vec<Token>>();

    let content = tokens_to_string(source, &tokens);
    let expected = r#"<div class="c1 c2">test</div>"#;
    assert_eq!(content, expected);
}

#[test]
fn accept_with_optimization_ignore_close_tag_3() {
    let source = r#"
        <div class="c1 c2">
        </div>
    "#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let options = CompilerOptions::default().with_optimization(true);
    let mut optimizer = html::HtmlOptimizer::new(&options);
    let mut accepted_tokens = vec![];
    for token in &tokens {
        if optimizer.accept(source, token) {
            accepted_tokens.push(*token);
        }
    }

    let content = tokens_to_string(source, &accepted_tokens);
    assert_eq!(content, r#"<div class="c1 c2"></div>"#)
}
