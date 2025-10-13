use crate::codegen::parser::tokenizer::Kind;
use crate::codegen::parser::tokenizer::Token;
use crate::codegen::parser::tokenizer::get_nth_token;
use winnow::stream::Stream as _;
use winnow::stream::TokenSlice;

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

/// check if the token is @@escaped in the source string
pub(in crate::codegen::parser::types) fn is_token_escaped<'a>(
    stream: &'a TokenSlice<Token>,
) -> bool {
    let first = get_nth_token(stream, 0);
    let send = get_nth_token(stream, 1);
    match (first, send) {
        (Some(first), Some(send)) if first.kind() == Kind::AT && send.kind() == Kind::AT => true,
        _ => false,
    }
}
