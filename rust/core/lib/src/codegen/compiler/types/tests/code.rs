#![cfg(test)]
use crate::codegen::types::Block;
use crate::codegen::types::Template;
use crate::types::result;
use crate::types::template::Kind;
use quote::quote;

#[test]
fn to_code_token_stream_simple() -> result::Result<()> {
    let raw_content = r#"@{
        let test=1;
        let test2=2;
    }"#;
    let template = Template::from(&raw_content, None, Kind::KHTML)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
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
    let template = Template::from(&raw_content, None, Kind::KHTML)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
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
    let template = Template::from(&raw_content, None, Kind::KHTML)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
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
