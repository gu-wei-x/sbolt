#![cfg(test)]
use crate::codegen::consts;
use crate::codegen::parser::template::Block;
use crate::codegen::parser::tokenizer::Tokenizer;
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
            Block::parse_content($content, start_token, &mut token_stream, false).unwrap();
        }
    };
    ($case_name:ident, $section_name:expr, $section_content:expr) => {
        #[test]
        fn $case_name() -> core::result::Result<(), error::Error> {
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
            let block = Block::parse_content(content, start_token, &mut token_stream, false)?;
            assert_eq!(block.name, Some($section_name.to_string()));
            assert_eq!(block.content(), $section_content);
            Ok(())
        }
    };
}

section_test_case!(test_parse_section, "test", "<div>test</div>");
section_test_case!(test_parse_section_no_name, "@section {<div>test</div>}");
section_test_case!(test_parse_section_no_brace, "@section test <div>test</div>");
