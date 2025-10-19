#![cfg(test)]
use winnow::stream::TokenSlice;

use crate::codegen::CompilerOptions;
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::parser::types::context;
use crate::codegen::parser::types::context::ParseContext;
use crate::codegen::types::Block;
use crate::codegen::types::Template;
use crate::types::result;
use crate::types::template;
use crate::types::template::Kind;

#[test]
#[should_panic]
fn to_content_panic() {
    let raw_content = r#"
<html>
   <div>Test</div>
</html>"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options).unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
    assert_eq!(root_span.blocks().len(), 1);

    // 0: section
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KCONTENT(_)));
    block.to_content();
}

#[test]
fn to_content() -> result::Result<()> {
    let raw_content = r#"
@section test1 {
   this is test1
}"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
    assert_eq!(root_span.blocks().len(), 1);
    assert_eq!(block.content().trim(), "");

    // 0: section
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KSECTION(_, _)));
    assert_eq!(block.content().trim(), "this is test1");
    let content_block = block.to_content();
    assert!(matches!(content_block, Block::KCONTENT(_)));
    assert_eq!(content_block.content().trim(), "this is test1");

    Ok(())
}

#[test]
#[should_panic]
fn parse_transition_block_section_invalid_format() {
    let source = r#"@section{}"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(
        context::Kind::KSECTION,
        template::Kind::KHTML,
        &options,
        source,
    );
    Block::parse_transition_block(&mut token_stream, &mut context).unwrap();
}

#[test]
#[should_panic]
fn parse_transition_block_section_no_section_name() {
    let source = r#"@section {}"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(
        context::Kind::KSECTION,
        template::Kind::KHTML,
        &options,
        source,
    );
    Block::parse_transition_block(&mut token_stream, &mut context).unwrap();
}

#[test]
#[should_panic]
fn parse_transition_block_section_not_within_curlybracket() {
    let source = r#"@section test()"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(
        context::Kind::KSECTION,
        template::Kind::KHTML,
        &options,
        source,
    );
    Block::parse_transition_block(&mut token_stream, &mut context).unwrap();
}
