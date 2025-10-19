#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::types::Block;
use crate::codegen::types::Span;
use crate::codegen::types::Template;
use crate::types::result;
use crate::types::template::Kind;
use quote::quote;

#[test]
#[should_panic]
fn to_render_token_stream() {
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
    block.to_render_token_stream().expect("wrong block type");
}

#[test]
fn to_render_token_stream_with_no_param() -> result::Result<()> {
    // @render
    let raw_content = r#"@render()"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);

    let block = &root_span.blocks()[0];
    let ts = block.to_render_token_stream()?;
    let expected = quote! {
        let default_section = context.get_default_section();
        match default_section {
            Some(content) => writer.write(&content),
            None => {
                return Err(sbolt::types::error::RuntimeError::NotFound("Default section not found".to_string(), "".to_string()))
            }
        }
    };

    assert_eq!(ts.to_string(), expected.to_string());
    Ok(())
}

#[test]
fn to_render_token_stream_with_single_param() -> result::Result<()> {
    let raw_content = r#"@render(test)"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);

    let block = &root_span.blocks()[0];
    let ts = block.to_render_token_stream()?;
    let expected = quote! {
        let section_name = "test";
        let sections = context.get_section(section_name);
        match sections {
            Some(contents) => {
              for content in contents {
                  writer.write(&content)
              }
            },
            None => {
               return Err(sbolt::types::error::RuntimeError::NotFound(format!("Section `{}` not found", section_name), "".to_string()))
            }
        }
    };

    assert_eq!(ts.to_string(), expected.to_string());
    Ok(())
}

#[test]
fn to_render_token_stream_with_two_params() -> result::Result<()> {
    let raw_content = r#"@render(test, false)"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);

    let block = &root_span.blocks()[0];
    let ts = block.to_render_token_stream()?;
    let expected = quote! {
        let section_name = "test";
        let is_required = "false".parse::<bool>().is_ok();
        let sections = context.get_section(section_name);
        match sections {
            Some(contents) => {
                for content in contents {
                    writer.write(&content)
                }
            },
            None if is_required => {
                return Err(sbolt::types::error::RuntimeError::NotFound(format!("Section `{}` not found", section_name), "".to_string()))
            },
            _ => { }
        }
    };

    assert_eq!(ts.to_string(), expected.to_string());
    Ok(())
}

#[test]
#[should_panic]
fn to_render_token_stream_with_more_than_two_params() {
    // this won't happen as parser will catch this error.
    let raw_content = "";
    let mut span = Span::new(raw_content);
    span.push_block(Block::new_content(Span::new(raw_content)));
    span.push_block(Block::new_content(Span::new(raw_content)));
    span.push_block(Block::new_content(Span::new(raw_content)));
    let render_block = Block::new_render(span);
    render_block
        .to_render_token_stream()
        .expect("wrong number of params");
}
