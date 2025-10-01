#![cfg(test)]
use crate::{
    codegen::parser::{template::block, tokenizer::Tokenizer},
    types::error,
};
use winnow::stream::TokenSlice;

#[test]
fn test_block_parse_empty_stream() {
    let source = "";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);

    let result = block::Block::parse(source, &mut token_stream);
    assert!(result.is_err());
}

// type from context.
#[test]
fn test_block_parse_content() -> core::result::Result<(), error::Error> {
    let source = r#"
         test
    "#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = block::Block::parse(source, &mut token_stream)?;
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 1);
    assert_eq!(block.blocks()[0].kind(), block::Kind::CONTENT);
    assert_eq!(block.blocks()[0].content().trim(), "test");
    Ok(())
}

#[test]
fn test_block_parse_inline_code() -> core::result::Result<(), error::Error> {
    // no ending.
    let source = r#"@test"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = block::Block::parse(source, &mut token_stream)?;
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 1);
    assert_eq!(block.blocks()[0].kind(), block::Kind::INLINEDCODE);
    assert_eq!(block.blocks()[0].content().trim(), "test");

    // ending with ;
    let source = r#"@test;"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = block::Block::parse(source, &mut token_stream)?;
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert_eq!(block.blocks().len(), 2);

    // 0: name, 1:;
    assert!(matches!(block.blocks()[0].kind(), block::Kind::INLINEDCODE));
    assert_eq!(block.blocks()[0].content().trim(), "test");
    assert!(matches!(block.blocks()[1].kind(), block::Kind::CONTENT));
    assert_eq!(block.blocks()[1].content().trim(), ";");
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
    let result = block::Block::parse(source, &mut token_stream);
    assert!(result.is_err());

    // Non-Empty.
    let source = "@{abc;}";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = block::Block::parse(source, &mut token_stream)?;
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert_eq!(block.blocks().len(), 1);
    assert_eq!(block.blocks()[0].kind(), block::Kind::CODE);
    assert_eq!(block.blocks()[0].content().trim(), "abc;");
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
    let block = block::Block::parse(source, &mut token_stream)?;

    // root.
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert_eq!(block.blocks().len(), 1);

    // l1,
    let block = &block.blocks()[0];
    assert_eq!(block.kind(), block::Kind::CODE);
    assert_eq!(block.blocks().len(), 3);

    // l1-sub-post(linefeed)
    // inside
    let blocks = block.blocks();
    assert_eq!(block.blocks().len(), 3);
    assert_eq!(blocks[0].kind(), block::Kind::CODE);
    assert_eq!(blocks[1].kind(), block::Kind::CONTENT);
    assert_eq!(blocks[2].kind(), block::Kind::CODE);

    // l2
    let block = &blocks[1];
    assert_eq!(block.kind(), block::Kind::CONTENT);
    assert_eq!(block.blocks().len(), 3);
    assert_eq!(block.blocks()[0].kind(), block::Kind::CONTENT);
    assert_eq!(block.blocks()[1].kind(), block::Kind::CODE);
    assert_eq!(block.blocks()[2].kind(), block::Kind::CONTENT);
    Ok(())
}

#[test]
fn test_block_parse_complex_content_block() -> core::result::Result<(), error::Error> {
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
    let block = block::Block::parse(source, &mut token_stream)?;

    // root.
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert_eq!(block.blocks().len(), 2);

    // pre, child, post: \n
    let blocks = block.blocks();
    assert!(matches!(blocks[0].kind(), block::Kind::CONTENT));
    assert!(matches!(blocks[1].kind(), block::Kind::CODE));
    Ok(())
}

// escape should share the same code logic as they all need escape.
// escape from content
#[test]
fn test_block_parse_escape_from_content() -> core::result::Result<(), error::Error> {
    let source = r#"@@root"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = block::Block::parse(source, &mut token_stream)?;
    println!("{:#?}", block);

    // root.
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert_eq!(block.blocks().len(), 1);

    //@@exp inside content, @@ => @, "@exp..." as content.
    Ok(())
}

// escape from code
#[test]
fn test_block_parse_escape_from_code() -> core::result::Result<(), error::Error> {
    let source = r#"@{@@root}"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = block::Block::parse(source, &mut token_stream)?;
    println!("{:#?}", block);

    // root.
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert_eq!(block.blocks().len(), 1);

    //@@exp inside code, @@ => @, "@exp..." as code, @ is pattern binding in rust so need to escape.
    Ok(())
}
