#![cfg(test)]
use crate::codegen::types::Block;
use crate::codegen::types::Template;
use crate::types::result;
use quote::quote;

// layout is special: can only be generated from root block.
#[test]
#[should_panic]
fn to_layout_token_stream_from_layout_block() {
    let raw_content = r#"@layout test::test1;"#;
    let template = Template::from(&raw_content, None).unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
    assert_eq!(root_span.blocks().len(), 1);
    let layout_block = &root_span.blocks()[0];
    assert!(matches!(layout_block, Block::KLAYOUT(_)));
    layout_block.to_token_stream(Some(block)).unwrap();
}

#[test]
#[should_panic]
fn to_layout_token_stream_from_layout_block2() {
    let raw_content = r#"@layout test::test1;"#;
    let template = Template::from(&raw_content, None).unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
    assert_eq!(root_span.blocks().len(), 1);
    let layout_block = &root_span.blocks()[0];
    assert!(matches!(layout_block, Block::KLAYOUT(_)));
    layout_block.generate_layout_token_stream().unwrap();
}

#[test]
fn generate_layout_token_stream_from_root_block() -> result::Result<()> {
    let raw_content = r#"@layout test::test1;"#;
    let template = Template::from(&raw_content, None)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
    assert_eq!(root_span.blocks().len(), 1);
    assert!(matches!(root_span.blocks()[0], Block::KLAYOUT(_)));
    let result = block.generate_layout_token_stream()?;
    assert!(result.is_some());
    let ts = result.unwrap();
    let expected = quote! {
        fn layout() -> Option<String> {
            Some("test::test1".to_string())
        }
    };
    assert_eq!(ts.to_string(), expected.to_string());
    Ok(())
}

#[test]
fn to_token_stream() -> result::Result<()> {
    let raw_content = r#"@use test::test1;"#;
    let template = Template::from(&raw_content, None)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
    assert_eq!(root_span.blocks().len(), 1);
    let use_block = &root_span.blocks()[0];
    assert!(matches!(use_block, Block::KUSE(_)));
    let ts = &use_block.to_token_stream(Some(block))?[0];
    let expected = quote! {
       use test::test1;
    };
    assert_eq!(ts.to_string(), expected.to_string());
    Ok(())
}
