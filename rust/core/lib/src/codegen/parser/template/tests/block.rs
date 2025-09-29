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
    assert_eq!(block.name(), None);
    assert!(!block.has_blocks());
    assert!(matches!(block.kind(), block::Kind::CODE));
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
    assert_eq!(block.name(), None);
    assert!(!block.has_blocks());
    assert!(matches!(block.kind(), block::Kind::CONTENT));
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
    assert_eq!(block.name(), None);
    assert!(block.has_blocks());
    assert!(matches!(block.kind(), block::Kind::CONTENT));
    assert!(matches!(block.blocks()[0].kind(), block::Kind::INLINEDCODE));

    // ending with ;
    let source = r#"@test;"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Content);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name(), None);
    assert_eq!(block.blocks().len(), 2);
    assert!(matches!(block.kind(), block::Kind::CONTENT));

    // 0: name, 1:;
    assert!(matches!(block.blocks()[0].kind(), block::Kind::INLINEDCODE));
    assert!(matches!(block.blocks()[1].kind(), block::Kind::CONTENT));
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
    assert_eq!(block.name(), None);
    assert!(!block.has_blocks());
    assert!(matches!(block.kind(), block::Kind::INLINEDCONTENT));

    // ending with ;, content is different it will cosume all tokens util linefeed.
    let source = r#"@test;\n"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Code);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name(), None);
    assert!(!block.has_blocks());
    assert!(matches!(block.kind(), block::Kind::INLINEDCONTENT));
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
    assert_eq!(block.name(), None);

    assert!(block.has_blocks());
    assert!(matches!(block.kind(), block::Kind::CONTENT));
    assert!(matches!(block.blocks()[0].kind(), block::Kind::CODE));
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
    assert!(result.is_err());

    // Non-Empty.
    let source = "@{abc;}";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Code);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name(), None);
    assert!(!block.has_blocks());
    assert!(matches!(block.kind(), block::Kind::CONTENT));
    Ok(())
}

// @{}
#[test]
fn test_block_parse_complex_code_block() -> core::result::Result<(), error::Error> {
    let source = r#"
        @{
           l1
           @{
                l2
                @{
                    l3
                }
           }
        }"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Content);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name(), None);
    assert!(matches!(block.kind(), block::Kind::CONTENT));

    // pre, child, post: \n
    assert_eq!(block.blocks().len(), 1);
    let blocks = block.blocks()[0].blocks();

    // l1.
    assert!(matches!(blocks[0].kind(), block::Kind::CODE));
    assert!(!blocks[0].has_blocks());
    assert_eq!(blocks[0].content().trim(), "l1");

    // after l1.
    assert!(matches!(blocks[1].kind(), block::Kind::CONTENT));
    assert_eq!(blocks[1].blocks().len(), 3);
    let l1_content_blocks = blocks[1].blocks();
    assert!(matches!(l1_content_blocks[0].kind(), block::Kind::CONTENT));
    assert!(matches!(l1_content_blocks[1].kind(), block::Kind::CODE));
    assert!(matches!(l1_content_blocks[2].kind(), block::Kind::CONTENT));

    // last contains linefeed.
    assert!(matches!(blocks[2].kind(), block::Kind::CODE));
    Ok(())
}

// @{}
#[test]
fn test_block_parse_complex_content_block() -> core::result::Result<(), error::Error> {
    let source = r#"
        @{
           l1
           @{
                l2
                @{
                    l3
                }
           }
        }"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let mut context = ParseContext::new(Context::Code);
    let block = block::Block::parse(source, &mut token_stream, &mut context)?;
    assert_eq!(block.name(), None);
    assert!(matches!(block.kind(), block::Kind::CONTENT));

    // pre, child, post: \n
    assert_eq!(block.blocks().len(), 3);
    let blocks = &block.blocks();

    // l1.
    assert!(matches!(blocks[0].kind(), block::Kind::CONTENT));
    assert!(!blocks[0].has_blocks());
    assert_eq!(blocks[0].content().trim(), "l1");

    // after l1.
    assert!(matches!(blocks[1].kind(), block::Kind::CODE));
    assert_eq!(blocks[1].blocks().len(), 3);
    let l1_content_blocks = blocks[1].blocks();
    assert!(matches!(l1_content_blocks[0].kind(), block::Kind::CODE));
    assert!(matches!(l1_content_blocks[1].kind(), block::Kind::CONTENT));
    assert!(matches!(l1_content_blocks[2].kind(), block::Kind::CODE));

    // last contains linefeed.
    assert!(matches!(blocks[2].kind(), block::Kind::CONTENT));
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
    assert_eq!(block.name(), None);
    // pre, child, post: \n
    assert_eq!(block.blocks().len(), 2);
    let blocks = block.blocks();
    assert!(matches!(blocks[0].kind(), block::Kind::CONTENT));
    assert!(matches!(blocks[1].kind(), block::Kind::CODE));
    Ok(())
}
