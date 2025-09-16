use crate::parser::tokenizer::token::Kind;
use crate::parser::tokenizer::token::Token;
use winnow::stream::LocatingSlice;
use winnow::stream::Stream;
use winnow::stream::TokenSlice;

pub(crate) type StrStream<'a> = LocatingSlice<&'a str>;
pub(crate) type TokenStream<'i> = TokenSlice<'i, Token>;

pub(crate) fn skip_whitespace(stream: &mut TokenStream) {
    skip_next_token_if(stream, |k| k == Kind::WHITESPACE);
}

pub(crate) fn skip_meanlingless(stream: &mut TokenStream) {
    skip_next_token_if(stream, |k| {
        vec![Kind::COMMENT, Kind::NEWLINE, Kind::WHITESPACE].contains(&k)
    });
}

pub(crate) fn skip_whitespace_and_newline(stream: &mut TokenStream) {
    skip_next_token_if(stream, |k| {
        vec![Kind::WHITESPACE, Kind::NEWLINE].contains(&k)
    });
}

pub(crate) fn get_next_token_if<'a, F: Fn(Kind) -> bool>(
    stream: &mut TokenSlice<'a, Token>,
    pred: F,
) -> Option<&'a Token> {
    while let Some(current_token) = stream.peek_token() {
        if pred(current_token.kind()) {
            stream.next_token();
        } else {
            break;
        }
    }

    stream.peek_token()
}

pub(crate) fn skip_next_token_if<F: Fn(Kind) -> bool>(stream: &mut TokenStream, pred: F) {
    while let Some(current_token) = stream.peek_token() {
        if pred(current_token.kind()) {
            stream.next_token();
        } else {
            break;
        }
    }
}
