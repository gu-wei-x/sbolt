#![cfg(test)]
use crate::{
    codegen::{
        consts,
        parser::{
            template::{Kind, ParseContext, block},
            tokenizer::Tokenizer,
        },
    },
    types::error,
};
use winnow::stream::TokenSlice;

macro_rules! parse_context_test_case {
    ($case_name:ident, $source:expr, $from_kind: expr) => {
        #[test]
        fn $case_name() -> core::result::Result<(), error::CompileError> {
            let source = $source;
            let tokenizer = Tokenizer::new(source);
            let tokens = tokenizer.into_vec();
            let mut token_stream = TokenSlice::new(&tokens);
            let context = ParseContext::new($from_kind);
            let result = context.switch_if_possible(source, &mut token_stream);
            assert!(result.is_err());
            Ok(())
        }
    };
    ($case_name:ident, $source:expr, $is_from_content:expr, $from_kind: expr, $expected: expr) => {
        #[test]
        fn $case_name() -> core::result::Result<(), error::CompileError> {
            let source = $source;
            let tokenizer = Tokenizer::new(source);
            let tokens = tokenizer.into_vec();
            let mut token_stream = TokenSlice::new(&tokens);
            let context = ParseContext::new($from_kind);
            let (should_switch, _) = context.switch_if_possible(source, &mut token_stream)?;
            assert_eq!(should_switch, $expected);
            Ok(())
        }
    };
}

// from content.
parse_context_test_case!(
    test_parse_context_from_content_should_switch,
    "@123",
    true,
    Kind::CONTENT,
    true
);
parse_context_test_case!(
    test_parse_context_from_content_single_at,
    "@",
    true,
    Kind::CONTENT,
    false
);
parse_context_test_case!(
    test_parse_context_from_content_should_not_switch,
    "@@123",
    true,
    Kind::CONTENT,
    false
);

parse_context_test_case!(
    test_parse_context_from_content_layout,
    &format!("@{}", consts::DIRECTIVE_KEYWORD_LAYOUT),
    true,
    Kind::ROOT,
    true
);

parse_context_test_case!(
    test_parse_context_from_content_section,
    &format!("@{}", consts::KEYWORD_SECTION),
    true,
    Kind::CONTENT,
    true
);

parse_context_test_case!(
    test_parse_context_from_content_use,
    &format!("@{}", consts::DIRECTIVE_KEYWORD_USE),
    true,
    Kind::CONTENT,
    true
);

parse_context_test_case!(
    test_parse_context_from_content_oparenthesis,
    "@(",
    true,
    Kind::CONTENT,
    true
);

parse_context_test_case!(
    test_parse_context_from_content_ocurlybracket,
    "@{",
    true,
    Kind::CONTENT,
    true
);

// from code.
parse_context_test_case!(
    test_parse_context_from_code_should_switch,
    "@123",
    false,
    Kind::CODE,
    true
);
parse_context_test_case!(
    test_parse_context_from_code_single_at,
    "@",
    false,
    Kind::CODE,
    false
);
parse_context_test_case!(
    test_parse_context_from_code_should_not_switch,
    "@@123",
    false,
    Kind::CODE,
    false
);

// not allowed.
parse_context_test_case!(
    test_parse_context_from_code_layout,
    &format!("@{}", consts::DIRECTIVE_KEYWORD_LAYOUT),
    Kind::CODE
);

parse_context_test_case!(
    test_parse_context_from_code_section,
    &format!("@{}", consts::KEYWORD_SECTION),
    false,
    Kind::CODE,
    true
);

// not allowed.
parse_context_test_case!(
    test_parse_context_from_code_use,
    &format!("@{}", consts::DIRECTIVE_KEYWORD_USE),
    Kind::CODE
);

parse_context_test_case!(
    test_parse_context_from_code_oparenthesis,
    "@(",
    false,
    Kind::CODE,
    true
);

parse_context_test_case!(
    test_parse_context_from_code_ocurlybracket,
    "@{",
    false,
    Kind::CODE,
    true
);

#[test]
fn test_parse_context_to_block_empty() -> core::result::Result<(), error::CompileError> {
    let source = "";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut context = ParseContext::new(Kind::ROOT);
    for token in tokens {
        context.push(token);
    }
    let block = context.consume(source);
    assert!(block.is_none());
    Ok(())
}

#[test]
fn test_parse_context_to_block_from_content() -> core::result::Result<(), error::CompileError> {
    let source = "test1 test2";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut context = ParseContext::new(Kind::ROOT);
    for token in tokens {
        context.push(token);
    }
    let result = context.consume(source);
    assert!(result.is_some());
    let block = result.unwrap();
    assert_eq!(block.content(), source);
    assert!(matches!(block.kind(), block::Kind::ROOT));
    Ok(())
}

#[test]
fn test_parse_context_to_block_from_code() -> core::result::Result<(), error::CompileError> {
    let source = "test1 test2";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut context = ParseContext::new(Kind::CODE);
    for token in tokens {
        context.push(token);
    }
    let result = context.consume(source);
    assert!(result.is_some());
    let block = result.unwrap();
    assert_eq!(block.content(), source);
    assert!(matches!(block.kind(), block::Kind::CODE));
    Ok(())
}
