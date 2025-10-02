use crate::codegen::parser::tokenizer;
use crate::codegen::parser::tokenizer::Token;
use crate::codegen::parser::tokenizer::stream::StrStream;
use winnow::stream::Stream as _;

pub(crate) struct Tokenizer<'a> {
    stream: StrStream<'a>,
    current_line: usize,
    current_column: usize,
    eof: bool,
}

impl<'a> Tokenizer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        // skip BOM if present.
        let mut stream = StrStream::new(input);
        if input.as_bytes().starts_with(&[0xEF, 0xBB, 0xBF]) {
            stream.next_slice(3);
        }
        Self {
            stream: StrStream::new(input),
            current_line: 0,
            current_column: 0,
            eof: false,
        }
    }

    pub(crate) fn into_vec(self) -> Vec<Token> {
        let capacity = core::cmp::min(
            self.stream.len(),
            usize::MAX / core::mem::size_of::<Token>(),
        );

        let mut tokens = Vec::with_capacity(capacity);
        tokens.extend(self);
        tokens
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            return None;
        }

        let token = tokenizer::token::tokenize(
            &mut self.stream,
            &mut self.current_line,
            &mut self.current_column,
        );
        match token.kind() {
            tokenizer::token::Kind::EOF => {
                self.eof = true;
                Some(token)
            }
            _ => Some(token),
        }
    }
}
