#![cfg(test)]
use crate::{
    codegen::{
        consts,
        parser::{
            template::{Context, ParseContext, block},
            tokenizer::Tokenizer,
        },
    },
    types::error,
};
use winnow::stream::{Stream as _, TokenSlice};

macro_rules! parse_context_test_case {
    ($case_name:ident, $source:expr, $is_from_content:expr, $expected: expr) => {
        #[test]
        fn $case_name() -> core::result::Result<(), error::Error> {
            let source = $source;
            let tokenizer = Tokenizer::new(source);
            let tokens = tokenizer.into_vec();
            let mut token_stream = TokenSlice::new(&tokens);
            let context = if $is_from_content {
                ParseContext::new(Context::Content)
            } else {
                ParseContext::new(Context::Code)
            };
            let token = token_stream.next_token().unwrap();
            let should_switch = context.should_switch(source, token, &mut token_stream)?;
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
    true
);
parse_context_test_case!(test_parse_context_from_content_single_at, "@", true, false);
parse_context_test_case!(
    test_parse_context_from_content_should_not_switch,
    "@@123",
    true,
    false
);

parse_context_test_case!(
    test_parse_context_from_content_layout,
    &format!("@{}", consts::DIRECTIVE_KEYWORD_LAYOUT),
    true,
    true
);

parse_context_test_case!(
    test_parse_context_from_content_section,
    &format!("@{}", consts::KEYWORD_SECTION),
    true,
    false
);

parse_context_test_case!(
    test_parse_context_from_content_use,
    &format!("@{}", consts::DIRECTIVE_KEYWORD_USE),
    true,
    true
);

parse_context_test_case!(
    test_parse_context_from_content_oparenthesis,
    "@(",
    true,
    true
);

parse_context_test_case!(
    test_parse_context_from_content_ocurlybracket,
    "@{",
    true,
    true
);

// from code.
parse_context_test_case!(
    test_parse_context_from_code_should_switch,
    "@123",
    false,
    true
);
parse_context_test_case!(test_parse_context_from_code_single_at, "@", false, false);
parse_context_test_case!(
    test_parse_context_from_code_should_not_switch,
    "@@123",
    false,
    false
);

parse_context_test_case!(
    test_parse_context_from_code_layout,
    &format!("@{}", consts::DIRECTIVE_KEYWORD_LAYOUT),
    false,
    false
);

parse_context_test_case!(
    test_parse_context_from_code_section,
    &format!("@{}", consts::KEYWORD_SECTION),
    false,
    false
);

parse_context_test_case!(
    test_parse_context_from_code_use,
    &format!("@{}", consts::DIRECTIVE_KEYWORD_USE),
    false,
    false
);

parse_context_test_case!(test_parse_context_from_code_oparenthesis, "@(", false, true);

parse_context_test_case!(
    test_parse_context_from_code_ocurlybracket,
    "@{",
    false,
    true
);

#[test]
fn test_parse_context_to_block_empty() -> core::result::Result<(), error::Error> {
    let source = "";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut context = ParseContext::new(Context::Content);
    for token in tokens {
        context.push(token);
    }
    let block = context.to_block(source);
    assert!(block.is_none());

    let mut context = ParseContext::new(Context::Content);
    let block = context.to_block(source);
    assert!(block.is_none());
    Ok(())
}

#[test]
fn test_parse_context_to_block_from_content() -> core::result::Result<(), error::Error> {
    let source = "test1 test2";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut context = ParseContext::new(Context::Content);
    for token in tokens {
        context.push(token);
    }
    let result = context.to_block(source);
    assert!(result.is_some());
    let block = result.unwrap();
    assert_eq!(block.content(), source);
    assert!(matches!(block.kind(), block::Kind::CONTENT));
    Ok(())
}

#[test]
fn test_parse_context_to_block_from_code() -> core::result::Result<(), error::Error> {
    let source = "test1 test2";
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut context = ParseContext::new(Context::Code);
    for token in tokens {
        context.push(token);
    }
    let result = context.to_block(source);
    assert!(result.is_some());
    let block = result.unwrap();
    assert_eq!(block.content(), source);
    assert!(matches!(block.kind(), block::Kind::CODE));
    Ok(())
}
