#![cfg(test)]
use crate::codegen::types::Block;
use crate::codegen::types::Span;
use crate::codegen::types::Template;
use crate::types::result;
use crate::types::template::Kind;
use quote::quote;

#[test]
#[should_panic]
fn to_content_token_stream_from_wrong_block() {
    let raw_content = r#"@layout test::test1"#;
    let template = Template::from(&raw_content, None, Kind::KHTML);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KLAYOUT(_)));
    block.to_content_token_stream().expect("wrong block type");
}

#[test]
fn to_content_token_stream_from_simple_content() -> result::Result<()> {
    let raw_content = r#"test::test1"#;
    let template = Template::from(&raw_content, None, Kind::KHTML)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KCONTENT(_)));
    let ts = &block.to_content_token_stream()?;
    let expected = quote! {
        writer.write("test::test1");
    };
    assert_eq!(ts.to_string(), expected.to_string());
    Ok(())
}

#[test]
#[should_panic]
fn to_inline_content_token_stream_from_wrong_block() {
    let raw_content = r#"@layout test::test1"#;
    let template = Template::from(&raw_content, None, Kind::KHTML);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KLAYOUT(_)));
    block
        .to_inline_content_token_stream()
        .expect("wrong block type");
}

#[test]
fn to_inline_content_token_stream_from_code_block() -> result::Result<()> {
    let raw_content = r#"@{123; @test}"#;
    let template = Template::from(&raw_content, None, Kind::KHTML)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let code_block = &root_span.blocks()[0];
    assert!(matches!(code_block, Block::KCODE(_)));

    let ts = code_block.to_token_stream(Some(&block))?;
    let expected = quote! {
        123;
        writer.write("test");
    };
    assert_eq!(ts[0].to_string(), expected.to_string());
    Ok(())
}

#[test]
#[should_panic]
fn to_inline_content_token_stream_from_inline_content_with_wrong_content() {
    // this won't happen as parser will catch this error.
    let raw_content = "";
    let mut span = Span::new(raw_content);
    span.push_block(Block::new_content(Span::new(raw_content)));
    span.push_block(Block::new_content(Span::new(raw_content)));
    let render_block = Block::new_inline_content(span);
    render_block
        .to_inline_content_token_stream()
        .expect("wrong content");
}
