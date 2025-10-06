#![cfg(test)]
use crate::codegen::consts;
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::types::Block;
use crate::types::error;
use winnow::stream::TokenSlice;

macro_rules! directive_test_case {
    ($name:ident, $directive:expr) => {
        #[test]
        fn $name() {
            let statements = [
                $directive,
                &format!("{} ", $directive),
                &format!("{};", $directive),
                &format!("{}\n", $directive),
                &format!("{} ;", $directive),
                &format!("{} \n", $directive),
                &format!("{} ;\n", $directive),
            ];
            for statement in statements {
                let tokenizer = Tokenizer::new(statement);
                let tokens = tokenizer.into_vec();
                let mut token_stream = TokenSlice::new(&tokens);
                let result = Block::parse_directive(statement, $directive, &mut token_stream);
                assert!(result.is_err());
            }
        }
    };
    ($name:ident, $statement:expr, $directive:expr, $type_func: expr) => {
        #[test]
        fn $name() -> core::result::Result<(), error::CompileError> {
            let statements = [
                &format!("{} {}", $directive, $statement),
                &format!("{} {};", $directive, $statement),
                &format!("{} {};\n", $directive, $statement),
                &format!("{} {}\n", $directive, $statement),
                &format!("{} {} ;", $directive, $statement),
                &format!("{} {} ;\n", $directive, $statement),
            ];
            for statement in statements {
                let tokenizer = Tokenizer::new(statement);
                let tokens = tokenizer.into_vec();
                let mut token_stream = TokenSlice::new(&tokens);
                let block = Block::parse_directive(statement, $directive, &mut token_stream)?;
                assert!($type_func(&block));
                assert_eq!(block.content().trim(), $statement);
            }

            Ok(())
        }
    };
}

// layout.
directive_test_case!(
    parse_illegal_directive_layout,
    consts::DIRECTIVE_KEYWORD_LAYOUT
);

directive_test_case!(
    parse_directive_layout,
    "abc:test",
    consts::DIRECTIVE_KEYWORD_LAYOUT,
    |b| matches!(b, &Block::KLAYOUT(_))
);

// use.
directive_test_case!(parse_illegal_directive_use, consts::DIRECTIVE_KEYWORD_USE);
directive_test_case!(
    parse_directive_use,
    "abc:test",
    consts::DIRECTIVE_KEYWORD_USE,
    |b| matches!(b, &Block::KUSE(_))
);
