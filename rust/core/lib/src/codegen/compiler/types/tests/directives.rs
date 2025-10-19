#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::types::Block;
use crate::codegen::types::Template;
use crate::types::result;
use crate::types::template::Kind;
use quote::quote;

// layout is special: can only be generated from root block.
#[test]
#[should_panic]
fn to_layout_token_stream_from_layout_block() {
    let raw_content = r#"@layout test::test1;"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options).unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let layout_block = &root_span.blocks()[0];
    assert!(matches!(layout_block, Block::KLAYOUT(_)));
    layout_block.to_token_stream(Some(block)).unwrap();
}

#[test]
#[should_panic]
fn to_layout_token_stream_from_layout_block2() {
    let raw_content = r#"@layout test::test1;"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options).unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let layout_block = &root_span.blocks()[0];
    assert!(matches!(layout_block, Block::KLAYOUT(_)));
    layout_block.generate_layout_token_stream().unwrap();
}

#[test]
fn generate_layout_token_stream_no_layout_block() -> result::Result<()> {
    let raw_content = r#"test"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    let result = block.generate_layout_token_stream()?;
    assert!(result.is_none());
    Ok(())
}

#[test]
fn generate_layout_token_stream_with_one_layout_block() -> result::Result<()> {
    let raw_content = r#"@layout test::test1;"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
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
#[should_panic]
fn generate_layout_token_stream_with_multiple_layout_blocks() {
    let raw_content = r#"@layout test::test1; @layout test::test2;"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    block
        .generate_layout_token_stream()
        .expect("Only 0/1 layout is allowed.");
}

#[test]
#[should_panic]
fn to_use_token_stream_from_wrong_block() {
    let raw_content = r#"@layout test::test1;"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    assert!(matches!(root_span.blocks()[0], Block::KLAYOUT(_)));
    block
        .to_use_token_stream()
        .expect("expected KUSE block here");
}

#[test]
#[should_panic]
fn to_use_token_stream_with_invalid_content() {
    let raw_content = r#"@use (abc;"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let use_block = &root_span.blocks()[0];
    assert!(matches!(use_block, Block::KUSE(_)));
    use_block
        .to_use_token_stream()
        .expect("expected valid use block here");
}

#[test]
fn to_token_stream() -> result::Result<()> {
    let raw_content = r#"@use test::test1;"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
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

#[test]
#[should_panic]
fn generate_imports_token_stream_from_wrong_block() {
    let raw_content = r#"@use test::test1;"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let use_block = &root_span.blocks()[0];
    assert!(matches!(use_block, Block::KUSE(_)));
    use_block
        .generate_imports_token_stream()
        .expect("expected KROOT block here");
}

#[test]
#[should_panic]
fn generate_imports_token_stream_with_wrong_content() {
    let raw_content = r#"
    @use test::test1;
    @use (abc;
    "#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    block
        .generate_imports_token_stream()
        .expect("expected valid use blocks here");
}
