#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::types;
use crate::codegen::types::Block;
use crate::codegen::types::Template;
use crate::types::result;
use crate::types::template::Kind;
use quote::quote;

#[test]
#[should_panic]
fn to_code_token_on_non_code_block() {
    let raw_content = r#"
        let test=1;
        let test2=2;
    "#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options).unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KCONTENT(_)));
    block.to_code_token_stream(Some(block)).unwrap();
}

#[test]
#[should_panic]
fn to_code_token_stream_with_no_from_block() {
    let raw_content = r#"@{
        let test=1;
        let test2=2;
    }"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KCODE(_)));
    block
        .to_code_token_stream(None)
        .expect("Expected from block here");
}

#[test]
#[should_panic]
fn to_code_token_stream_with_invalid_content() {
    let raw_content = r#"@{
        let test=1;
        abc);
        let test2=2;
    }"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KCODE(_)));
    block
        .to_code_token_stream(Some(block))
        .expect("Expected valid code block here");
}

#[test]
fn to_code_token_stream_simple() -> result::Result<()> {
    let raw_content = r#"@{
        let test=1;
        let test2=2;
    }"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KCODE(_)));
    let ts = &block.to_code_token_stream(Some(block))?;
    let expected = quote! {
        let test=1;
        let test2=2;
    };
    assert_eq!(ts.to_string(), expected.to_string());
    Ok(())
}

#[test]
fn to_code_token_stream_with_block() -> result::Result<()> {
    let raw_content = r#"@{
        for i in 0..5 {
            @{<tr>}
        }
    }"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let code_block = &root_span.blocks()[0];
    assert!(matches!(code_block, Block::KCODE(_)));
    let ts = block.to_token_stream(Some(block))?;
    let ts = quote! { #(#ts)* };
    let expected = quote! {
        for i in 0..5 {
            writer.write("<tr>");
        }
    };
    assert_eq!(ts.to_string(), expected.to_string());
    Ok(())
}

#[test]
fn to_code_token_stream_with_complex_nested_block() -> result::Result<()> {
    let raw_content = r#"@{
        for i in 0..5 {
            @{<tr>}
            for j in 0..5 {
                @{<td>@j</td>}
            }
            @{</tr>}
        }
    }"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let code_block = &root_span.blocks()[0];
    assert!(matches!(code_block, Block::KCODE(_)));
    let ts = block.to_token_stream(Some(block))?;
    let ts = quote! { #(#ts)* };
    let expected = quote! {
        for i in 0..5 {
            writer.write("<tr>");
            for j in 0 .. 5 {
                writer.write("<td>");
                writer.write(&j.to_string());
                writer.write("</td>");
            }
            writer.write("</tr>");
        }
    };
    assert_eq!(ts.to_string(), expected.to_string());
    Ok(())
}

#[test]
#[should_panic]
fn to_inline_code_token_stream_from_wrong_block() {
    let raw_content = r#"test"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let block = &root_span.blocks()[0];
    block
        .to_inline_code_token_stream()
        .expect("wrong block type");
}

#[test]
fn to_inline_code_token_stream_from_content_block() -> result::Result<()> {
    let raw_content = r#"@{testcode;@{@name}}"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let code_block = &root_span.blocks()[0];
    assert!(matches!(code_block, Block::KCODE(_)));

    let ts = code_block.to_token_stream(Some(&block))?;
    let expected = quote! {
       testcode;
       writer.write(&name.to_string());
    };
    assert_eq!(ts[0].to_string(), expected.to_string());
    Ok(())
}

#[test]
#[should_panic]
fn to_inline_code_token_stream_with_with_wrong_content() {
    // this won't happen as parser will catch this error.
    let raw_content = "";
    let mut span = types::Span::new(raw_content);
    span.push_block(Block::new_content(types::Span::new(raw_content)));
    span.push_block(Block::new_content(types::Span::new(raw_content)));
    let render_block = Block::new_inline_code(span);
    render_block
        .to_inline_code_token_stream()
        .expect("Expected valid inline code block here");
}
