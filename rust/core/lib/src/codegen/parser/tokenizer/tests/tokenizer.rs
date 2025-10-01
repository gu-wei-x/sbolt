#![cfg(test)]
macro_rules! tokenizer_test_case {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            use crate::codegen::parser::tokenizer;
            use crate::codegen::parser::tokenizer::Tokenizer;
            use crate::codegen::parser::tokenizer::token;

            // Join the input strings to form a single string for tokenization.
            let tokenizer_input: String = $input.join("");
            let tokenizer = Tokenizer::new(&tokenizer_input);
            let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
            assert_eq!(tokens.len(), $expected.len());
            for (i, token) in tokens.iter().enumerate() {
                // kind.
                assert_eq!(token.kind(), $expected[i]);

                // range.
                match token.kind() {
                    tokenizer::Kind::EOF => {
                        assert_eq!(tokenizer_input.len()..tokenizer_input.len(), token.range());
                    }
                    _ => {
                        assert_eq!($input[i], &tokenizer_input[token.range()]);
                    }
                }
            }
        }
    };
}

tokenizer_test_case!(test_tokenizer_eof, vec![""], [token::Kind::EOF]);

tokenizer_test_case!(
    test_tokenizer_symbols,
    vec![
        "@", "=", "!", "-", "<", ">", "{", "}", "(", ")", "/", "*", ";", "\r\n", "\n"
    ],
    vec![
        token::Kind::AT,
        token::Kind::EQUALS,
        token::Kind::EXCLAMATION,
        token::Kind::HYPHEN,
        token::Kind::LESSTHAN,
        token::Kind::GREATTHAN,
        token::Kind::OCURLYBRACKET,
        token::Kind::CCURLYBRACKET,
        token::Kind::OPARENTHESIS,
        token::Kind::CPARENTHESIS,
        token::Kind::SLASH,
        token::Kind::ASTERISK,
        token::Kind::SEMICOLON,
        token::Kind::NEWLINE,
        token::Kind::NEWLINE,
        token::Kind::EOF
    ]
);

tokenizer_test_case!(
    test_tokenizer_exp,
    vec!["abc"],
    [token::Kind::EXPRESSION, token::Kind::EOF]
);

tokenizer_test_case!(
    test_tokenizer_exp_with_symbol,
    vec!["abc", ";"],
    [
        token::Kind::EXPRESSION,
        token::Kind::SEMICOLON,
        token::Kind::EOF
    ]
);

tokenizer_test_case!(
    test_tokenizer_unicode_stream,
    vec!["你好", ";", "hello", ")", "世界"],
    [
        token::Kind::EXPRESSION,
        token::Kind::SEMICOLON,
        token::Kind::EXPRESSION,
        token::Kind::CPARENTHESIS,
        token::Kind::EXPRESSION,
        token::Kind::EOF
    ]
);
