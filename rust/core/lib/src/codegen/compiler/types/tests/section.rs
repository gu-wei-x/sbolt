#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::compiler::context::CodeGenContext;
use crate::codegen::types::Block;
use crate::codegen::types::Template;
use crate::types::result;
use crate::types::template::Kind;
use quote::quote;

#[test]
#[should_panic]
fn to_section_token_stream_from_wrong_block() {
    let raw_content = r#"test"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let context = CodeGenContext::new(Kind::KHTML, &options);
    block
        .to_section_token_stream(&context)
        .expect("Expect section block here");
}

#[test]
fn to_section_token_stream_simple() -> result::Result<()> {
    let raw_content = r#"@section test{test}"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);

    let block = &root_span.blocks()[0];
    let context = CodeGenContext::new(Kind::KHTML, &options);
    let ts = block.to_section_token_stream(&context)?;
    let expected = quote! {
        let section_name = "test";
        let inner_writer = {
            let mut writer = self.create_writer(None);
            writer.write("test");
            writer
        };
        context.add_section(section_name, inner_writer.into_string());
    };

    assert_eq!(ts.to_string(), expected.to_string());

    Ok(())
}

#[test]
fn to_section_token_stream_composite() -> result::Result<()> {
    let raw_content = r#"
    @section test{@test helloworld}"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);

    let block = &root_span.blocks()[0];
    let context = CodeGenContext::new(Kind::KHTML, &options);
    let ts = block.to_section_token_stream(&context)?;
    let expected = quote! {
        let section_name = "test";
        let section_writer = {
            let mut writer = self.create_writer(None);
            writer.write(&test.to_string());
            writer.write(" helloworld");
            writer
        };
        context.add_section(section_name, section_writer.into_string());
    };

    assert_eq!(ts.to_string(), expected.to_string());

    Ok(())
}
