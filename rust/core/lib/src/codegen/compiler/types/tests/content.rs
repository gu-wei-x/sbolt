#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::compiler::context::CodeGenContext;
use crate::codegen::types::Block;
use crate::codegen::types::Span;
use crate::codegen::types::Template;
use crate::types::result;
use crate::types::template::Kind;
use quote::quote;

fn generate_content(
    source: &str,
    kind: Kind,
    optimize: bool,
) -> result::Result<(proc_macro2::TokenStream, crate::codegen::StaticOutputStats)> {
    let options = CompilerOptions::default().with_optimization(optimize);
    let template = Template::from(source, None, kind, &options)?;
    let context = CodeGenContext::new(kind, &options);
    let root = template.block();
    let tokens = root.to_token_stream(Some(root), &context)?;
    Ok((quote! { #(#tokens)* }, context.static_output_stats()))
}

#[test]
fn merges_static_output_and_measures_utf8_literal_savings() -> result::Result<()> {
    let source = "<div    class = \"\u{e9}\"  >a@* comment *@b</div>";
    let expected = "<div class=\"\u{e9}\">ab</div>";
    let (tokens, stats) = generate_content(source, Kind::KHTML, true)?;
    assert_eq!(
        tokens.to_string(),
        quote! { writer.write(#expected); }.to_string()
    );
    assert_eq!(stats.input_writes(), 2);
    assert_eq!(stats.output_writes(), 1);
    assert_eq!(stats.input_bytes(), source.len() - "@* comment *@".len());
    assert_eq!(stats.output_bytes(), expected.len());
    assert_eq!(stats.distinct_output_bytes(), expected.len());
    assert!(stats.output_bytes() < stats.input_bytes());

    let (_, unoptimized) = generate_content(source, Kind::KHTML, false)?;
    assert_eq!(unoptimized.input_bytes(), unoptimized.output_bytes());
    assert_eq!(unoptimized.output_writes(), 2);
    Ok(())
}

#[test]
fn merges_partial_tags_without_changing_values() -> result::Result<()> {
    let source = "<div title=\"a@* comment *@b\"   > hi  there </div>";
    let expected = "<div title=\"ab\"> hi  there </div>";
    let (tokens, _) = generate_content(source, Kind::KHTML, true)?;
    assert_eq!(
        tokens.to_string(),
        quote! { writer.write(#expected); }.to_string()
    );
    Ok(())
}

#[test]
fn merged_attributes_retain_hidden_element_separators() -> result::Result<()> {
    let source = "<span>Hello</span>\n<div hid@* join *@den></div><span>world</span>";
    let expected = "<span>Hello</span>\n<div hidden></div><span>world</span>";
    let (tokens, _) = generate_content(source, Kind::KHTML, true)?;
    assert_eq!(
        tokens.to_string(),
        quote! { writer.write(#expected); }.to_string()
    );
    Ok(())
}

#[test]
fn block_whitespace_reduction_saves_embedded_literal_bytes() -> result::Result<()> {
    let source = "<div>\n  <p>hello</p>\n</div>";
    let expected = "<div><p>hello</p></div>";
    let (tokens, stats) = generate_content(source, Kind::KHTML, true)?;
    assert_eq!(
        tokens.to_string(),
        quote! { writer.write(#expected); }.to_string()
    );
    assert_eq!(stats.input_bytes() - stats.output_bytes(), 4);
    let (tokens, stats) = generate_content(source, Kind::KHTML, false)?;
    assert_eq!(
        tokens.to_string(),
        quote! { writer.write(#source); }.to_string()
    );
    assert_eq!(stats.input_bytes(), stats.output_bytes());
    Ok(())
}

#[test]
fn dynamic_boundaries_preserve_whitespace() -> result::Result<()> {
    let source = "<span>hello</span>\n @value<div>\n  </div>";
    let (tokens, _) = generate_content(source, Kind::KHTML, true)?;
    let expected = quote! {
        writer.write("<span>hello</span>\n ");
        writer.write(&value.to_string());
        writer.write("<div>\n  </div>");
    };
    assert_eq!(tokens.to_string(), expected.to_string());
    Ok(())
}

#[test]
fn aggregate_metrics_deduplicate_literals_across_templates() -> result::Result<()> {
    let mut total = crate::codegen::CompileResult::new();
    for source in [
        "<div   class = 'x'>text</div>",
        "<div class='x'>te@*join*@xt</div>",
    ] {
        let (_, stats) = generate_content(source, Kind::KHTML, true)?;
        let mut result = crate::codegen::CompileResult::new();
        result.add_static_output_stats(stats);
        result.merge_into(&mut total);
    }
    let stats = total.static_output_stats();
    let literal = "<div class='x'>text</div>";
    assert_eq!(stats.output_bytes(), literal.len() * 2);
    assert_eq!(stats.distinct_output_bytes(), literal.len());
    assert_eq!(stats.input_writes(), 3);
    assert_eq!(stats.output_writes(), 2);
    Ok(())
}

#[test]
fn raw_output_and_rust_code_are_html_context_barriers() -> result::Result<()> {
    let source = "<div title=\"@value\"><span   class = 'x'> tail  </span>";
    let (tokens, stats) = generate_content(source, Kind::KHTML, true)?;
    let expected = quote! {
        writer.write("<div title=\"");
        writer.write(&value.to_string());
        writer.write("\"><span   class = 'x'> tail  </span>");
    };
    assert_eq!(tokens.to_string(), expected.to_string());
    assert_eq!(stats.unknown_context_writes(), 1);

    let source = "@{writer.write(\"<script>\");@{<span   class = 'x'> hi </span>}}";
    let (tokens, _) = generate_content(source, Kind::KHTML, true)?;
    let expected = quote! {
        writer.write("<script>");
        writer.write("<span   class = 'x'> hi </span>");
    };
    assert_eq!(tokens.to_string(), expected.to_string());
    Ok(())
}

#[test]
fn optimization_leaves_json_and_text_generation_unchanged() -> result::Result<()> {
    let source = "<div   class = 'x'>a@* comment *@b</div>";
    for kind in [Kind::KJSON, Kind::KTEXT] {
        let (before, _) = generate_content(source, kind, false)?;
        let (after, _) = generate_content(source, kind, true)?;
        assert_eq!(before.to_string(), after.to_string());
    }
    Ok(())
}

#[test]
#[should_panic]
fn to_content_token_stream_from_wrong_block() {
    let raw_content = r#"@layout test::test1"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KLAYOUT(_)));
    let context = CodeGenContext::new(Kind::KHTML, &options);
    block
        .to_content_token_stream(&context)
        .expect("wrong block type");
}

#[test]
fn to_content_token_stream_from_simple_content() -> result::Result<()> {
    let raw_content = r#"test::test1"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KCONTENT(_)));
    let context = CodeGenContext::new(Kind::KHTML, &options);
    let ts = &block.to_content_token_stream(&context)?;
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
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KLAYOUT(_)));
    let context = CodeGenContext::new(Kind::KHTML, &options);
    block
        .to_inline_content_token_stream(&context)
        .expect("wrong block type");
}

#[test]
fn to_inline_content_token_stream_from_code_block() -> result::Result<()> {
    let raw_content = r#"@{123; @test}"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let code_block = &root_span.blocks()[0];
    assert!(matches!(code_block, Block::KCODE(_)));

    let context = CodeGenContext::new(Kind::KHTML, &options);
    let ts = code_block.to_token_stream(Some(&block), &context)?;
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
    let options = CompilerOptions::default();
    let context = CodeGenContext::new(Kind::KHTML, &options);
    render_block
        .to_inline_content_token_stream(&context)
        .expect("wrong content");
}
