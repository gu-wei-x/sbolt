use crate::codegen::parser::tokenizer::Token;
use crate::codegen::parser::tokenizer::token::Kind;
use winnow::stream::LocatingSlice;
use winnow::stream::Stream;
use winnow::stream::TokenSlice;

pub(crate) type StrStream<'a> = LocatingSlice<&'a str>;
pub(crate) type TokenStream<'i> = TokenSlice<'i, Token>;

pub(crate) fn skip_newline(stream: &mut TokenStream) -> bool {
    skip_next_token_if(stream, |k| k == Kind::NEWLINE)
}

pub(crate) fn skip_whitespace(stream: &mut TokenStream) -> bool {
    skip_next_token_if(stream, |k| k == Kind::WHITESPACE)
}

pub(crate) fn skip_whitespace_and_newline(stream: &mut TokenStream) -> bool {
    skip_next_token_if(stream, |k| {
        vec![Kind::WHITESPACE, Kind::NEWLINE].contains(&k)
    })
}

#[allow(dead_code)]
pub(crate) fn get_next_token_if<'a, F: Fn(Kind) -> bool>(
    stream: &mut TokenSlice<'a, Token>,
    skip_pred: F,
) -> Option<&'a Token> {
    while let Some(current_token) = stream.peek_token() {
        if skip_pred(current_token.kind()) && current_token.kind() != Kind::EOF {
            stream.next_token();
        } else {
            break;
        }
    }

    stream.peek_token()
}

#[allow(dead_code)]
pub(crate) fn get_next_token_util<'a, F: Fn(Kind) -> bool>(
    stream: &mut TokenSlice<'a, Token>,
    pred: F,
) -> Option<&'a Token> {
    get_next_token_if(stream, |k| !pred(k))
}

pub(crate) fn skip_next_token_if<F: Fn(Kind) -> bool>(stream: &mut TokenStream, pred: F) -> bool {
    let mut skipped = false;
    while let Some(current_token) = stream.peek_token() {
        if pred(current_token.kind()) {
            stream.next_token();
            skipped = true;
        } else {
            break;
        }
    }
    skipped
}

pub(crate) fn get_nth_token<'a>(
    stream: &TokenSlice<'a, Token>,
    offset: usize,
) -> Option<&'a Token> {
    match stream.iter_offsets().nth(offset) {
        Some((_, token)) if token.kind() == Kind::EOF => None,
        Some((_, token)) => Some(token),
        None => None,
    }
}
