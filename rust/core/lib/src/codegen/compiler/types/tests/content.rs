#![cfg(test)]
use crate::codegen::types::Block;
use crate::codegen::types::Template;
use crate::types::result;
use quote::quote;

#[test]
fn to_content_token_stream_from_simple_content() -> result::Result<()> {
    let raw_content = r#"test::test1"#;
    let template = Template::from(&raw_content, None)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
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
