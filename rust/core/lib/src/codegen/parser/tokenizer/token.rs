use winnow::stream::AsBStr as _;
use winnow::stream::ContainsToken as _;
use winnow::stream::Location as _;
use winnow::stream::Stream as _;

use crate::codegen::parser::tokenizer::Token;
use crate::codegen::parser::tokenizer::stream::StrStream;
use crate::types::Location;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum Kind {
    NEWLINE = 0,
    EXPRESSION = 1,
    EOF = 2,
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
    ASTERISK = b'*',
    WHITESPACE = b' ',
    COLON = b':',
    SEMICOLON = b';',
    COMMA = b',',
    DQMARK = b'"',
    SQMAERK = b'\'',
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token({:?}, {}, {})",
            self.kind(),
            self.range().start,
            self.range().end
        )
    }
}

pub(crate) fn tokenize(stream: &mut StrStream<'_>, location: &mut Location) -> Token {
    let Some(peeked_byte) = stream.as_bstr().first() else {
        let start = stream.current_token_start();
        let loc = Location::new(location.line, start - location.column);
        let token = Token::new(Kind::EOF, start, start, loc);
        return token;
    };

    let token = match peeked_byte {
        b'@' => tokenize_symbol(stream, Kind::AT, location),
        b'=' => tokenize_symbol(stream, Kind::EQUALS, location),
        b'!' => tokenize_symbol(stream, Kind::EXCLAMATION, location),
        b'-' => tokenize_symbol(stream, Kind::HYPHEN, location),
        b'<' => tokenize_symbol(stream, Kind::LESSTHAN, location),
        b'>' => tokenize_symbol(stream, Kind::GREATTHAN, location),
        b')' => tokenize_symbol(stream, Kind::CPARENTHESIS, location),
        b'(' => tokenize_symbol(stream, Kind::OPARENTHESIS, location),
        b'}' => tokenize_symbol(stream, Kind::CCURLYBRACKET, location),
        b'{' => tokenize_symbol(stream, Kind::OCURLYBRACKET, location),
        b'/' => tokenize_symbol(stream, Kind::SLASH, location),
        b'*' => tokenize_symbol(stream, Kind::ASTERISK, location),
        b':' => tokenize_symbol(stream, Kind::COLON, location),
        b';' => tokenize_symbol(stream, Kind::SEMICOLON, location),
        b',' => tokenize_symbol(stream, Kind::COMMA, location),
        b'"' => tokenize_symbol(stream, Kind::DQMARK, location),
        b'\'' => tokenize_symbol(stream, Kind::SQMAERK, location),
        b' ' => tokenize_whitespace(stream, location),
        b'\r' => tokenize_newline(stream, location),
        b'\n' => tokenize_newline(stream, location),
        _ => tokenize_expression(stream, location),
    };

    token
}

fn tokenize_symbol(stream: &mut StrStream<'_>, token_type: Kind, location: &Location) -> Token {
    let start = stream.current_token_start();

    // symbol is a single character token.
    let offset = 1;
    stream.next_slice(offset);

    let end = stream.previous_token_end();
    let loc = Location::new(location.line, start - location.column);
    Token::new(token_type, start, end, loc)
}

fn tokenize_whitespace(stream: &mut StrStream<'_>, location: &Location) -> Token {
    let start = stream.current_token_start();
    let offset = stream
        .as_bstr()
        .offset_for(|b| !&(b' ', b'\t').contains_token(b))
        .unwrap_or(stream.eof_offset());
    stream.next_slice(offset);
    let end = stream.previous_token_end();
    let loc = Location::new(location.line, start - location.column);
    Token::new(Kind::WHITESPACE, start, end, loc)
}

fn tokenize_newline(stream: &mut StrStream<'_>, location: &mut Location) -> Token {
    let start = stream.current_token_start();
    let mut offset = '\r'.len_utf8();
    let has_lf = stream.as_bstr().get(1) == Some(&b'\n');
    if has_lf {
        offset += '\n'.len_utf8();
    }
    stream.next_slice(offset);
    let end = stream.previous_token_end();
    let loc = Location::new(location.line, start - location.column);
    location.line += 1;
    location.column = end;
    Token::new(Kind::NEWLINE, start, end, loc)
}

fn tokenize_expression(stream: &mut StrStream<'_>, location: &Location) -> Token {
    let start = stream.current_token_start();
    const TOKEN_START: &[u8] = b"@=!-<>(){}/*,; :\"'\r\n";
    let offset = stream
        .as_bstr()
        .offset_for(|b| TOKEN_START.contains_token(b))
        .unwrap_or_else(|| stream.eof_offset());
    stream.next_slice(offset);
    let end = stream.previous_token_end();
    let loc = Location::new(location.line, start - location.column);
    Token::new(Kind::EXPRESSION, start, end, loc)
}
