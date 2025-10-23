#![cfg(test)]
use crate::{
    codegen::{
        CompilerOptions,
        compiler::context::CodeGenContext,
        types::{Block, Span, Template},
    },
    types::{result, template::Kind},
};

#[test]
#[should_panic]
fn to_token_stream_from_empty() {
    // this won't happen as parser will catch this error.
    let span = Span::new("");
    let root_block = Block::new_root(span);
    let options = CompilerOptions::default();
    let context = CodeGenContext::new(Kind::KHTML, &options);
    root_block
        .to_token_stream(Some(&root_block), &context)
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
    let context = CodeGenContext::new(Kind::KHTML, &options);
    root_span.blocks()[0]
        .generate_render_token_stream(&context)
        .expect("Expect Root block here");
}

#[test]
#[should_panic]
fn to_token_stream_from_function() {
    // this won't happen as parser will catch this error.
    let span = Span::new("");
    let root_block = Block::new_functions(span);
    let options = CompilerOptions::default();
    let context = CodeGenContext::new(Kind::KHTML, &options);
    root_block
        .to_token_stream(Some(&root_block), &context)
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
    let context = CodeGenContext::new(Kind::KHTML, &options);
    block.to_token_stream(Some(&block), &context)?;
    Ok(())
}
