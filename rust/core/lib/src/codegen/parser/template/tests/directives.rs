#![cfg(test)]
use crate::codegen::consts;
use crate::codegen::parser::template::{Block, Kind, ParseContext};
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::types::error;
use winnow::stream::TokenSlice;

macro_rules! directive_test_case {
    ($name:ident, $directive:expr, $kind:expr) => {
        #[test]
        fn $name() {
            let contents = [
                $directive,
                &format!("{} ", $directive),
                &format!("{};", $directive),
                &format!("{}\n", $directive),
                &format!("{} ;", $directive),
                &format!("{} \n", $directive),
                &format!("{} ;\n", $directive),
            ];

            for content in contents {
                let statement = &format!("@{}", content);
                let tokenizer = Tokenizer::new(statement);
                let tokens = tokenizer.into_vec();
                let mut token_stream = TokenSlice::new(&tokens);
                let result = Block::parse_at_block(
                    statement,
                    &mut token_stream,
                    &mut ParseContext::new($kind),
                );
                assert!(result.is_err());
            }
        }
    };
    ($name:ident, $statement:expr, $directive:expr, $kind:expr) => {
        #[test]
        fn $name() -> core::result::Result<(), error::CompileError> {
            let contents = [
                &format!("{} {}", $directive, $statement),
                &format!("{} {};", $directive, $statement),
                &format!("{} {};\n", $directive, $statement),
                &format!("{} {}\n", $directive, $statement),
                &format!("{} {} ;", $directive, $statement),
                &format!("{} {} ;\n", $directive, $statement),
            ];

            for content in contents {
                let statement = &format!("@{}", content);
                let tokenizer = Tokenizer::new(statement);
                let tokens = tokenizer.into_vec();
                let mut token_stream = TokenSlice::new(&tokens);
                let block = Block::parse_at_block(
                    statement,
                    &mut token_stream,
                    &mut ParseContext::new($kind),
                )?;
                assert_eq!(block.name(), Some(&$directive.to_string()));
                assert_eq!(block.kind(), $kind);
                assert_eq!(block.content().trim(), $statement);
            }

            Ok(())
        }
    };
}

// layout.

directive_test_case!(
    test_parse_directive_layout_illegal,
    consts::DIRECTIVE_KEYWORD_LAYOUT,
    Kind::DIRECTIVE
);
directive_test_case!(
    test_parse_directive_layout,
    "abc:test",
    consts::DIRECTIVE_KEYWORD_LAYOUT,
    Kind::DIRECTIVE
);

// use.
directive_test_case!(
    test_parse_directive_use_illegal,
    consts::DIRECTIVE_KEYWORD_USE,
    Kind::DIRECTIVE
);
directive_test_case!(
    test_parse_directive_use,
    "abc:test",
    consts::DIRECTIVE_KEYWORD_USE,
    Kind::DIRECTIVE
);
