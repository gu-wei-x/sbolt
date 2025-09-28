#![cfg(test)]
use crate::{
    codegen::parser::{
        template::{Context, ParseContext, block},
        tokenizer::Tokenizer,
    },
    types::error,
};
use winnow::stream::TokenSlice;

#[test]
fn test_block_parse_empty_stream() {
    let source = "";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);

    // code context.
    let mut code_context = ParseContext::new(Context::Code);
    let result = block::Block::parse(source, &mut token_stream, &mut code_context);
    assert!(result.is_err());

    // content context.
    let mut content_context = ParseContext::new(Context::Content);
    let result = block::Block::parse(source, &mut token_stream, &mut content_context);
    assert!(result.is_err());
}

// type from context.
#[test]
fn test_block_parse_implicit_code() -> core::result::Result<(), error::Error> {
    let source = r#"
         test;
    "#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Code);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name, None);
    assert!(block.blocks.is_empty());
    assert!(matches!(block.span.kind(), block::Kind::CODE(_)));
    Ok(())
}

#[test]
fn test_block_parse_implicit_content() -> core::result::Result<(), error::Error> {
    let source = r#"
         test;
    "#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Content);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name, None);
    assert!(block.blocks.is_empty());
    assert!(matches!(block.span.kind(), block::Kind::CONTENT(_)));
    Ok(())
}

#[test]
fn test_block_parse_inline_code() -> core::result::Result<(), error::Error> {
    // no ending.
    let source = r#"@test"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Content);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name, None);
    assert!(block.blocks.is_empty());
    assert!(matches!(block.span.kind(), block::Kind::INLINEDCODE(_)));

    // ending with ;
    let source = r#"@test;"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Content);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name, None);
    assert_eq!(block.blocks.len(), 2);
    assert!(matches!(block.span.kind(), block::Kind::CONTENT(_)));

    // 0: name, 1:;
    assert!(matches!(
        block.blocks[0].span.kind(),
        block::Kind::INLINEDCODE(_)
    ));
    assert!(matches!(
        block.blocks[1].span.kind(),
        block::Kind::CONTENT(_)
    ));
    Ok(())
}

// @exp
#[test]
fn test_block_parse_inline_content() -> core::result::Result<(), error::Error> {
    // no ending.
    let source = r#"@test"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Code);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name, None);
    assert!(block.blocks.is_empty());
    assert!(matches!(block.span.kind(), block::Kind::INLINEDCONTENT(_)));

    // ending with ;, content is different it will cosume all tokens util linefeed.
    let source = r#"@test;\n"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Code);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name, None);
    assert!(block.blocks.is_empty());
    assert!(matches!(block.span.kind(), block::Kind::INLINEDCONTENT(_)));
    Ok(())
}

// @{}
#[test]
fn test_block_parse_code_block() -> core::result::Result<(), error::Error> {
    // Empty.
    let source = "@{}";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Content);
    let result = block::Block::parse(source, &mut token_stream, &mut context);
    assert!(result.is_err());

    // Non-Empty.
    let source = "@{abc;}";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Content);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name, None);
    assert!(block.blocks.is_empty());
    assert!(matches!(block.span.kind(), block::Kind::CODE(_)));
    Ok(())
}

// @{}
#[test]
fn test_block_parse_content_block() -> core::result::Result<(), error::Error> {
    // Empty.
    let source = "@{}";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Code);
    let result = block::Block::parse(source, &mut token_stream, &mut context);
    println!("************{result:?}");
    assert!(result.is_err());

    // Non-Empty.
    let source = "@{abc;}";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Code);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name, None);
    assert!(block.blocks.is_empty());
    assert!(matches!(block.span.kind(), block::Kind::CONTENT(_)));
    Ok(())
}

// @{}
#[test]
fn test_block_parse_complex_code_block() -> core::result::Result<(), error::Error> {
    let source = r#"
        @{
           parent
           @{
                child1
                @{
                    child2
                }
           }
        }"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Content);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name, None);
    // pre, child, post: \n
    assert_eq!(block.blocks.len(), 3);
    Ok(())
}

// @{}
#[test]
fn test_block_parse_complex_content_block() -> core::result::Result<(), error::Error> {
    let source = r#"
        @{
           parent
           @{
                child1
                @{
                    child2
                }
           }
        }"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Code);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name, None);
    // pre, child, post: \n
    assert_eq!(block.blocks.len(), 3);
    Ok(())
}

#[test]
fn test_block_parse_complex_content_block2() -> core::result::Result<(), error::Error> {
    let source = r#"
         root
        @{
           parent
           @{
                child1
                @{
                    child2
                }
           }
        }"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Content);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name, None);
    // pre, child, post: \n
    assert_eq!(block.blocks.len(), 2);
    Ok(())
}
