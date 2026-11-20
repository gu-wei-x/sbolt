#![cfg(test)]
use crate::{
    codegen::{
        CompilerOptions,
        compiler::context::CodeGenContext,
        types::{Block, Span, Template},
    },
    types::{result, template::Kind},
};
use quote::quote;

#[test]
fn default_interpolation_borrows_the_whole_expression() -> result::Result<()> {
    let options = CompilerOptions::default();
    let source = "@(values[index + 1])";
    let template = Template::from(source, None, Kind::KTEXT, &options)?;
    let mut context = CodeGenContext::new(Kind::KTEXT, &options);
    let tokens = template
        .block()
        .to_token_stream(Some(template.block()), &mut context)?;
    let expected = quote! {
        sbolt::types::WriteToRef(&(values[index + 1])).__sbolt_write_to(&mut writer);
    };
    assert_eq!(quote! { #(#tokens)* }.to_string(), expected.to_string());
    Ok(())
}

#[test]
fn layout_content_preserves_unknown_insertion_context() -> result::Result<()> {
    let source = "@layout layout;\n<div   id='x'>text</div>";
    let options = CompilerOptions::default().with_optimization(true);
    let template = Template::from(source, None, Kind::KHTML, &options)?;
    let mut context = CodeGenContext::new(Kind::KHTML, &options);
    let tokens = template
        .block()
        .generate_render_token_stream(&mut context)?;
    let write = quote! { writer.write("<div   id='x'>text</div>"); };
    assert!(tokens.to_string().contains(&write.to_string()));
    assert_eq!(context.static_output_stats().unknown_context_writes(), 1);
    Ok(())
}

#[test]
#[should_panic]
fn to_token_stream_from_empty() {
    // this won't happen as parser will catch this error.
    let span = Span::new("");
    let root_block = Block::new_root(span);
    let options = CompilerOptions::default();
    let mut context = CodeGenContext::new(Kind::KHTML, &options);
    root_block
        .to_token_stream(Some(&root_block), &mut context)
        .expect("Root block is empty, should panic");
}

#[test]
#[should_panic]
fn generate_render_token_stream_from_non_root() {
    let raw_content = r#"test"#;
    let options = CompilerOptions::default().with_mod_name("test_mod");
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let mut context = CodeGenContext::new(Kind::KHTML, &options);
    root_span.blocks()[0]
        .generate_render_token_stream(&mut context)
        .expect("Expect Root block here");
}

#[test]
#[should_panic]
fn to_token_stream_from_function() {
    // this won't happen as parser will catch this error.
    let span = Span::new("");
    let root_block = Block::new_functions(span);
    let options = CompilerOptions::default();
    let mut context = CodeGenContext::new(Kind::KHTML, &options);
    root_block
        .to_token_stream(Some(&root_block), &mut context)
        .expect("Not implemented yet, should panic");
}

#[test]
fn to_token_stream_from_comments() -> result::Result<()> {
    // not implemented yet, do nothing here.
    let raw_content = r#"@*test*@"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));

    let options = CompilerOptions::default();
    let mut context = CodeGenContext::new(Kind::KHTML, &options);
    block.to_token_stream(Some(&block), &mut context)?;
    Ok(())
}
