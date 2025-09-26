#![cfg(test)]
use crate::codegen::consts;
use crate::codegen::parser::template::Block;
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::types::error;
use winnow::stream::{Stream, TokenSlice};

macro_rules! directive_test_case {
    ($name:ident, $directive:expr) => {
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
                let start_token = token_stream.peek_token().unwrap();
                let result = Block::parse_code(statement, start_token, &mut token_stream);
                assert!(result.is_err());
            }
        }
    };
    ($name:ident, $statement:expr, $directive:expr) => {
        #[test]
        fn $name() -> core::result::Result<(), error::Error> {
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
                let start_token = token_stream
                    .peek_token()
                    .ok_or_else(|| error::Error::from_parser(None, "Expected '@'"))?;
                let block = Block::parse_code(statement, start_token, &mut token_stream)?;
                assert_eq!(block.name, Some($directive.to_string()));
                assert_eq!(block.content(), $statement);
            }

            Ok(())
        }
    };
}

// layout.
directive_test_case!(
    test_parse_directive_layout_illegal,
    consts::DIRECTIVE_KEYWORD_LAYOUT
);
directive_test_case!(
    test_parse_directive_layout,
    "abc:test",
    consts::DIRECTIVE_KEYWORD_LAYOUT
);

// use.
directive_test_case!(
    test_parse_directive_use_illegal,
    consts::DIRECTIVE_KEYWORD_USE
);
directive_test_case!(
    test_parse_directive_use,
    "abc:test",
    consts::DIRECTIVE_KEYWORD_USE
);
