pub mod stream;
pub mod token;

use stream::StrStream;
use winnow::stream::AsBStr as _;
use winnow::stream::ContainsToken as _;
use winnow::stream::FindSlice as _;
use winnow::stream::Location;
use winnow::stream::Stream as _;

pub struct Tokenizer<'a> {
    stream: StrStream<'a>,
    eof: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        // skip BOM if present.
        let mut stream = StrStream::new(input);
        if input.as_bytes().starts_with(&[0xEF, 0xBB, 0xBF]) {
            stream.next_slice(3);
        }
        Self {
            stream: StrStream::new(input),
            eof: false,
        }
    }

    pub fn into_vec(self) -> Vec<token::Token> {
        let capacity = core::cmp::min(
            self.stream.len(),
            usize::MAX / core::mem::size_of::<token::Token>(),
        );

        let mut tokens = Vec::with_capacity(capacity);
        tokens.extend(self);
        tokens
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = token::Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            return None;
        }

        let token = token::tokenize(&mut self.stream);
        match token.kind {
            token::Kind::EOF => {
                self.eof = true;
                Some(token)
            }
            _ => Some(token),
        }
    }
}
