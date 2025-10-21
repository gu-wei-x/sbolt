#![cfg(test)]
use crate::codegen::parser::tokenizer;
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::parser::tokenizer::token;
use crate::types::Location;
use std::io::Write;

macro_rules! tokenizer_test_case {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
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

tokenizer_test_case!(tokenizer_eof, vec![""], [token::Kind::EOF]);

tokenizer_test_case!(
    tokenizer_symbols,
    vec![
        "@", "=", "!", "-", "<", ">", "{", "}", "(", ")", "/", "*", ";", ",", ":", "\"", "'",
        "\r\n", "\n"
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
        token::Kind::COMMA,
        token::Kind::COLON,
        token::Kind::DQMARK,
        token::Kind::SQMAERK,
        token::Kind::NEWLINE,
        token::Kind::NEWLINE,
        token::Kind::EOF
    ]
);

tokenizer_test_case!(
    tokenizer_exp,
    vec!["abc"],
    [token::Kind::EXPRESSION, token::Kind::EOF]
);

tokenizer_test_case!(
    tokenizer_exp_with_symbol,
    vec!["abc", ";", ","],
    [
        token::Kind::EXPRESSION,
        token::Kind::SEMICOLON,
        token::Kind::COMMA,
        token::Kind::EOF
    ]
);

tokenizer_test_case!(
    tokenizer_unicode_stream,
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

tokenizer_test_case!(
    tokenizer_unicode_stream_and_lines,
    vec!["你好", "\n", "hello", "*", "world", "\r", "世界", "\r\n"],
    [
        token::Kind::EXPRESSION,
        token::Kind::NEWLINE,
        token::Kind::EXPRESSION,
        token::Kind::ASTERISK,
        token::Kind::EXPRESSION,
        token::Kind::NEWLINE,
        token::Kind::EXPRESSION,
        token::Kind::NEWLINE,
        token::Kind::EOF
    ]
);

#[test]
fn tokenizer_token_display() {
    let source = "\n123@*\n";
    let tokenizer = Tokenizer::new(source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let token = &tokens[1];
    let fmt_str = &token.to_string();
    assert_eq!(fmt_str, "Token(EXPRESSION, 1, 4)");
}

#[test]
fn tokenizer_stream_with_lines() {
    let token_parts = vec!["你好", "\n", "hello", "*", "world", "\r", "世界", "\r\n"];
    let tokenizer_input: String = token_parts.join("");
    let tokenizer = Tokenizer::new(&tokenizer_input);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    assert_eq!(tokens.len(), token_parts.len() + 1); // +1 for EOF token.

    // first token.
    let index = 0;
    let token = &tokens[index];
    assert_eq!(token.kind(), token::Kind::EXPRESSION);
    assert_eq!(&tokenizer_input[token.range()], token_parts[index]);
    assert_eq!(token.location(), Location::new(0, 0));

    // second token.
    let index = 1;
    let token = &tokens[index];
    assert_eq!(token.kind(), token::Kind::NEWLINE);
    assert_eq!(&tokenizer_input[token.range()], token_parts[index]);
    assert_eq!(token.location(), Location::new(0, 6));

    // third token.
    let index = 2;
    let token = &tokens[index];
    assert_eq!(token.kind(), token::Kind::EXPRESSION);
    assert_eq!(&tokenizer_input[token.range()], token_parts[index]);
    assert_eq!(token.location(), Location::new(1, 0));

    // fourth token.
    let index = 3;
    let token = &tokens[index];
    assert_eq!(token.kind(), token::Kind::ASTERISK);
    assert_eq!(&tokenizer_input[token.range()], token_parts[index]);
    assert_eq!(token.location(), Location::new(1, 5));

    // fifth token.
    let index = 4;
    let token = &tokens[index];
    assert_eq!(token.kind(), token::Kind::EXPRESSION);
    assert_eq!(&tokenizer_input[token.range()], token_parts[index]);
    assert_eq!(token.location(), Location::new(1, 6));

    // sixth token.
    let index = 5;
    let token = &tokens[index];
    assert_eq!(token.kind(), token::Kind::NEWLINE);
    assert_eq!(&tokenizer_input[token.range()], token_parts[index]);
    assert_eq!(token.location(), Location::new(1, 11));

    // seventh token.
    let index = 6;
    let token = &tokens[index];
    assert_eq!(token.kind(), token::Kind::EXPRESSION);
    assert_eq!(&tokenizer_input[token.range()], token_parts[index]);
    assert_eq!(token.location(), Location::new(2, 0));

    // eighth token.
    let index = 7;
    let token = &tokens[index];
    assert_eq!(token.kind(), token::Kind::NEWLINE);
    assert_eq!(&tokenizer_input[token.range()], token_parts[index]);
    assert_eq!(token.location(), Location::new(2, 6));

    // ninth token (EOF).
    let index = 8;
    let token = &tokens[index];
    assert_eq!(token.kind(), token::Kind::EOF);
    assert_eq!(tokenizer_input.len()..tokenizer_input.len(), token.range());
    assert_eq!(token.location(), Location::new(3, 0));
}

#[test]
fn tokenizer_stream_with_bom() {
    let mut buffer = Vec::<u8>::new();
    _ = buffer.write_all(&[0xEF, 0xBB, 0xBF]);
    _ = buffer.write_all(b"123@*\n");
    let source = String::from_utf8(buffer).expect("Invalid UTF-8 sequence");
    let tokenizer = Tokenizer::new(&source);
    let tokens: Vec<tokenizer::Token> = tokenizer.into_vec();
    let token = &tokens[1];
    assert_eq!(token.kind(), token::Kind::AT);
    assert_eq!(token.location(), Location::new(0, 6));
}
