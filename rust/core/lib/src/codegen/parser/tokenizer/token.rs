use winnow::stream::AsBStr as _;
use winnow::stream::ContainsToken as _;
use winnow::stream::Location;
use winnow::stream::Stream as _;

use crate::codegen::parser::tokenizer::Token;
use crate::codegen::parser::tokenizer::stream::StrStream;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum Kind {
    NEWLINE = 0,
    EXPRESSION = 1,
    EOF = 2,
    UNKNOWN = 3,
    AT = b'@',
    EQUALS = b'=',
    EXCLAMATION = b'!',
    HYPHEN = b'-',
    LESSTHAN = b'<',
    GREATTHAN = b'>',
    CPARENTHESIS = b')',
    OPARENTHESIS = b'(',
    CCURLYBRACKET = b'}',
    OCURLYBRACKET = b'{',
    SLASH = b'/',
    STAR = b'*',
    WHITESPACE = b' ',
    SEMICOLON = b';',
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({:?}, {}, {})", self.kind, self.start, self.end)
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: Kind::UNKNOWN,
            start: 0,
            end: 0,
        }
    }
}

pub(crate) fn tokenize(stream: &mut StrStream<'_>) -> Token {
    let Some(peeked_byte) = stream.as_bstr().first() else {
        let start = stream.current_token_start();
        let token = Token::new(Kind::EOF, start, start);
        return token;
    };

    let token = match peeked_byte {
        b'@' => tokenize_symbol(stream, Kind::AT),
        b'=' => tokenize_symbol(stream, Kind::EQUALS),
        b'!' => tokenize_symbol(stream, Kind::EXCLAMATION),
        b'-' => tokenize_symbol(stream, Kind::HYPHEN),
        b'<' => tokenize_symbol(stream, Kind::LESSTHAN),
        b'>' => tokenize_symbol(stream, Kind::GREATTHAN),
        b')' => tokenize_symbol(stream, Kind::CPARENTHESIS),
        b'(' => tokenize_symbol(stream, Kind::OPARENTHESIS),
        b'}' => tokenize_symbol(stream, Kind::CCURLYBRACKET),
        b'{' => tokenize_symbol(stream, Kind::OCURLYBRACKET),
        b'/' => tokenize_symbol(stream, Kind::SLASH),
        b'*' => tokenize_symbol(stream, Kind::STAR),
        b';' => tokenize_symbol(stream, Kind::SEMICOLON),
        b' ' => tokenize_whitespace(stream),
        b'\r' => tokenize_newline(stream),
        b'\n' => tokenize_newline(stream),
        _ => tokenize_expression(stream),
    };

    token
}

fn tokenize_symbol(stream: &mut StrStream<'_>, token_type: Kind) -> Token {
    let start = stream.current_token_start();

    // symbol is a single character token.
    let offset = 1;
    stream.next_slice(offset);

    let end = stream.previous_token_end();
    Token::new(token_type, start, end)
}

fn tokenize_whitespace(stream: &mut StrStream<'_>) -> Token {
    let start = stream.current_token_start();
    let offset = stream
        .as_bstr()
        .offset_for(|b| !&(b' ', b'\t').contains_token(b))
        .unwrap_or(stream.eof_offset());
    stream.next_slice(offset);
    let end = stream.previous_token_end();
    Token::new(Kind::WHITESPACE, start, end)
}

fn tokenize_newline(stream: &mut StrStream<'_>) -> Token {
    let start = stream.current_token_start();
    let mut offset = '\r'.len_utf8();
    let has_lf = stream.as_bstr().get(1) == Some(&b'\n');
    if has_lf {
        offset += '\n'.len_utf8();
    }
    stream.next_slice(offset);
    let end = stream.previous_token_end();
    Token::new(Kind::NEWLINE, start, end)
}

fn tokenize_expression(stream: &mut StrStream<'_>) -> Token {
    let start = stream.current_token_start();
    const TOKEN_START: &[u8] = b"@=!-<>(){}/*; \r\n";
    let offset = stream
        .as_bstr()
        .offset_for(|b| TOKEN_START.contains_token(b))
        .unwrap_or_else(|| stream.eof_offset());
    stream.next_slice(offset);
    let end = stream.previous_token_end();
    Token::new(Kind::EXPRESSION, start, end)
}
