#![cfg(test)]
use crate::codegen::parser::template::Block;
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::{consts, parser::template::block};
use crate::types::error;
use winnow::stream::{Stream, TokenSlice};

macro_rules! section_test_case {
    ($case_name:ident, $content:expr) => {
        #[test]
        #[should_panic]
        fn $case_name() {
            let tokenizer = Tokenizer::new($content);
            let tokens = tokenizer.into_vec();
            let mut token_stream = TokenSlice::new(&tokens);
            let start_token = token_stream.peek_token().unwrap();
            Block::parse_section($content, start_token, &mut token_stream).unwrap();
        }
    };
    ($case_name:ident, $section_name:expr, $section_content:expr) => {
        #[test]
        fn $case_name() -> core::result::Result<(), error::CompileError> {
            let content = &format!(
                "@{} {}{{{}}}",
                consts::KEYWORD_SECTION,
                $section_name,
                $section_content
            );
            let tokenizer = Tokenizer::new(content);
            let tokens = tokenizer.into_vec();
            let mut token_stream = TokenSlice::new(&tokens);
            let start_token = token_stream.peek_token().unwrap();
            // consume @
            token_stream.next_token();
            let block = Block::parse_section(content, start_token, &mut token_stream)?;
            assert_eq!(block.name(), Some(&$section_name.to_string()));
            assert_eq!(block.content(), $section_content);
            Ok(())
        }
    };
}

section_test_case!(test_parse_section, "test", "<div>test</div>");
section_test_case!(test_parse_section_no_name, "@section {<div>test</div>}");
section_test_case!(test_parse_section_no_brace, "@section test <div>test</div>");

// comments.
#[test]
fn test_block_parse_comment() -> core::result::Result<(), error::CompileError> {
    let source = r#"@****test****@"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let token = token_stream.next_token().unwrap();
    let block = block::Block::parse_comment(source, token, &mut token_stream)?;

    // root.
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::COMMENT);
    assert_eq!(block.content(), source);
    Ok(())
}

#[test]
#[should_panic]
fn test_block_parse_comment_without_closing() {
    let source = r#"@****test****"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let token = token_stream.next_token().unwrap();
    block::Block::parse_comment(source, token, &mut token_stream).unwrap();
}
