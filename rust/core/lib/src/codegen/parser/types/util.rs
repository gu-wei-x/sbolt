use crate::codegen::parser::tokenizer::Kind;
use crate::codegen::parser::tokenizer::Token;
use winnow::stream::Stream as _;
use winnow::stream::TokenSlice;

pub(in crate::codegen::parser::types) fn get_tokens_before<'a, F: Fn(Kind) -> bool>(
    stream: &'a mut TokenSlice<Token>,
    kind_fn: F,
) -> Option<Vec<Token>> {
    let mut tokens = vec![];
    while let Some(token) = stream.peek_token() {
        match token.kind() {
            Kind::EOF => break,
            kind if kind_fn(kind) => {
                break;
            }
            _ => {
                tokens.push(*token);
                stream.next_token();
            }
        }
    }

    Some(tokens)
}

pub(in crate::codegen::parser::types) fn get_token_if<'a, F: Fn(Kind) -> bool>(
    stream: &'a mut TokenSlice<Token>,
    kind_fn: F,
) -> Option<&'a Token> {
    let token = stream.peek_token();
    match token {
        Some(t) if kind_fn(t.kind()) => {
            stream.next_token();
            Some(t)
        }
        _ => None,
    }
}
