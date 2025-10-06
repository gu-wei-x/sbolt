#![cfg(test)]
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::types::Block;
use crate::types::result;
use winnow::stream::TokenSlice;

#[test]
fn block_parse_empty_stream() {
    let source = "";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);

    let result = Block::parse(source, &mut token_stream);
    assert!(result.is_err());
}

#[test]
fn block_parse_content() -> result::Result<()> {
    let source = r#"
         test
    "#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = Block::parse(source, &mut token_stream)?;
    assert!(matches!(block, Block::KROOT(_)));
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 1);
            let first_block = &span.blocks()[0];
            assert!(matches!(first_block, Block::KCONTENT(_)));
            assert_eq!(first_block.content().trim(), source.trim());
        }
        _ => panic!("Expected KROOT block"),
    }

    Ok(())
}

#[test]
fn block_parse_inline_code() -> result::Result<()> {
    // no ending.
    let source = r#"@test"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = Block::parse(source, &mut token_stream)?;
    assert!(matches!(block, Block::KROOT(_)));
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 1);
            let first_block = &span.blocks()[0];
            assert!(matches!(first_block, Block::KINLINEDCODE(_)));
            assert_eq!(first_block.content().trim(), "test");
        }
        _ => panic!("Expected KROOT block"),
    }

    // ending with ;
    let source = r#"@test;"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = Block::parse(source, &mut token_stream)?;
    assert!(matches!(block, Block::KROOT(_)));
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 2);
            let first_block = &span.blocks()[0];
            assert!(matches!(first_block, Block::KINLINEDCODE(_)));
            assert_eq!(first_block.content(), "test");

            let second_block = &span.blocks()[1];
            assert!(matches!(second_block, Block::KCONTENT(_)));
            assert_eq!(second_block.content(), ";");
        }
        _ => panic!("Expected KROOT block"),
    }

    Ok(())
}

// @{}
#[test]
fn block_parse_code_block() -> result::Result<()> {
    // Empty.
    let source = "@{}";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = Block::parse(source, &mut token_stream)?;
    assert!(matches!(block, Block::KROOT(_)));
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 1);
            let first_block = &span.blocks()[0];
            assert!(matches!(first_block, Block::KCODE(_)));
            assert!(first_block.content().is_empty());
        }
        _ => panic!("Expected KROOT block"),
    }

    // Non-Empty.
    let source = "@{abc;}";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = Block::parse(source, &mut token_stream)?;
    assert!(matches!(block, Block::KROOT(_)));
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 1);
            let first_block = &span.blocks()[0];
            assert!(matches!(first_block, Block::KCODE(_)));
            assert_eq!(first_block.content(), "abc;");
        }
        _ => panic!("Expected KROOT block"),
    }
    Ok(())
}

// @{}
#[test]
fn block_parse_complex_code_block() -> result::Result<()> {
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
    let block = Block::parse(source, &mut token_stream)?;

    // root.
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
    assert_eq!(root_span.blocks().len(), 1);

    // l1,
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KCODE(_)));
    let l1_span = match block {
        Block::KCODE(span) => span,
        _ => panic!("Expected KCODE block"),
    };
    assert_eq!(l1_span.blocks().len(), 3);
    assert!(matches!(l1_span.blocks()[0], Block::KCODE(_)));
    assert_eq!(l1_span.blocks()[0].content().trim(), "l1");
    assert!(matches!(l1_span.blocks()[1], Block::KCONTENT(_)));
    // newline
    assert!(matches!(l1_span.blocks()[2], Block::KCODE(_)));

    // l2,
    let block = &l1_span.blocks()[1];
    let l2_span = match block {
        Block::KCONTENT(span) => span,
        _ => panic!("Expected KCONTENT block"),
    };
    assert_eq!(l2_span.blocks().len(), 3);
    assert!(matches!(l2_span.blocks()[0], Block::KCONTENT(_)));
    assert_eq!(l2_span.blocks()[0].content().trim(), "l2");
    assert!(matches!(l2_span.blocks()[1], Block::KCODE(_)));
    // newline
    assert!(matches!(l2_span.blocks()[2], Block::KCONTENT(_)));

    // l3: leaf node
    let block = &l2_span.blocks()[1];
    let l3_span = match block {
        Block::KCODE(span) => span,
        _ => panic!("Expected KCODE block"),
    };
    assert_eq!(l3_span.blocks().len(), 0);
    assert_eq!(l3_span.content().trim(), "l3");

    Ok(())
}

#[test]
fn block_parse_complex_content_block() -> result::Result<()> {
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
    let block = Block::parse(source, &mut token_stream)?;

    // root.
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
    assert_eq!(root_span.blocks().len(), 2);

    // pre, child, post: \n
    let blocks = root_span.blocks();
    assert!(matches!(blocks[0], Block::KCONTENT(_)));
    assert!(matches!(blocks[1], Block::KCODE(_)));
    Ok(())
}

// escape from content
#[test]
fn block_parse_escape_from_content() -> result::Result<()> {
    let source = r#"@@root"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = Block::parse(source, &mut token_stream)?;

    // root.
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
    assert_eq!(root_span.blocks().len(), 1);

    let blocks = root_span.blocks();
    assert!(matches!(blocks[0], Block::KCONTENT(_)));
    assert_eq!(blocks[0].content(), "@root");

    Ok(())
}

// escape from code, @ is valid in Rust as pattern binding so need to escape.
#[test]
fn block_parse_escape_from_code() -> result::Result<()> {
    let source = r#"@{@@root}"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let block = Block::parse(source, &mut token_stream)?;

    // root.
    // root.
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
    assert_eq!(root_span.blocks().len(), 1);

    let blocks = root_span.blocks();
    assert!(matches!(blocks[0], Block::KCODE(_)));
    assert_eq!(blocks[0].content(), "@root");

    Ok(())
}
