use crate::{
    codegen::parser::json::object::JObject,
    types::{Location, error::CompileError, result},
};

const MAX_DEPTH: usize = 128;

#[derive(Debug)]
pub(in crate::codegen::parser::json) struct StateMachine<'s> {
    source: &'s str,
    position: usize,
}

impl<'s> StateMachine<'s> {
    pub(in crate::codegen::parser::json) fn new(source: &'s str) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    pub(in crate::codegen::parser::json) fn process(&mut self) -> result::Result<JObject> {
        let value = self.parse_value(0)?;
        self.skip_whitespace();
        if self.peek().is_some() {
            return Err(self.error("Unexpected content after JSON value"));
        }
        Ok(value)
    }

    fn parse_value(&mut self, depth: usize) -> result::Result<JObject> {
        self.skip_whitespace();
        match self.peek() {
            Some(b'{' | b'[') if depth >= MAX_DEPTH => {
                Err(self.error("JSON nesting exceeds 128 containers"))
            }
            Some(b'{') => self.parse_map(depth + 1),
            Some(b'[') => self.parse_array(depth + 1),
            Some(b'"') => Ok(JObject::KValue(self.parse_string()?)),
            Some(b'-' | b'0'..=b'9') => self.parse_number(),
            Some(b't') => self.parse_literal("true"),
            Some(b'f') => self.parse_literal("false"),
            Some(b'n') => self.parse_literal("null"),
            _ => Err(self.error("Expected a JSON value")),
        }
    }

    fn parse_map(&mut self, depth: usize) -> result::Result<JObject> {
        self.position += 1;
        let mut items = Vec::new();
        self.skip_whitespace();
        if self.consume(b'}') {
            return Ok(JObject::KMap(items));
        }
        loop {
            self.skip_whitespace();
            let name = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':', "Expected ':' after JSON member name")?;
            let value = self.parse_value(depth)?;
            items.push((name, value));
            self.skip_whitespace();
            if self.consume(b'}') {
                return Ok(JObject::KMap(items));
            }
            self.expect(b',', "Expected ',' or '}' after JSON member")?;
        }
    }

    fn parse_array(&mut self, depth: usize) -> result::Result<JObject> {
        self.position += 1;
        let mut items = Vec::new();
        self.skip_whitespace();
        if self.consume(b']') {
            return Ok(JObject::KARRAY(items));
        }
        loop {
            items.push(self.parse_value(depth)?);
            self.skip_whitespace();
            if self.consume(b']') {
                return Ok(JObject::KARRAY(items));
            }
            self.expect(b',', "Expected ',' or ']' after JSON array item")?;
        }
    }

    fn parse_string(&mut self) -> result::Result<String> {
        let start = self.position;
        self.expect(b'"', "Expected a double-quoted JSON string")?;
        while let Some(byte) = self.peek() {
            match byte {
                b'"' => {
                    self.position += 1;
                    return Ok(self.source[start..self.position].into());
                }
                b'\\' => {
                    self.position += 1;
                    match self.peek() {
                        Some(b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't') => {
                            self.position += 1;
                        }
                        Some(b'u') => {
                            self.position += 1;
                            for _ in 0..4 {
                                if !self.peek().is_some_and(|byte| byte.is_ascii_hexdigit()) {
                                    return Err(self.error("Expected four hex digits after '\\u'"));
                                }
                                self.position += 1;
                            }
                        }
                        _ => return Err(self.error("Invalid JSON string escape")),
                    }
                }
                0..=0x1f => return Err(self.error("Unescaped control character in JSON string")),
                _ => self.position += 1,
            }
        }
        Err(self.error("Unterminated JSON string"))
    }

    fn parse_number(&mut self) -> result::Result<JObject> {
        let start = self.position;
        self.consume(b'-');
        if !self.consume(b'0') {
            if !matches!(self.peek(), Some(b'1'..=b'9')) {
                return Err(self.error("Expected a digit in JSON number"));
            }
            self.consume_digits();
        }
        if self.consume(b'.') {
            self.require_digits()?;
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.position += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.position += 1;
            }
            self.require_digits()?;
        }
        Ok(JObject::KValue(self.source[start..self.position].into()))
    }

    fn parse_literal(&mut self, literal: &str) -> result::Result<JObject> {
        if !self.source[self.position..].starts_with(literal) {
            return Err(self.error("Invalid JSON literal"));
        }
        self.position += literal.len();
        Ok(JObject::KValue(literal.into()))
    }

    fn require_digits(&mut self) -> result::Result<()> {
        let start = self.position;
        self.consume_digits();
        if start == self.position {
            return Err(self.error("Expected a digit in JSON number"));
        }
        Ok(())
    }

    fn consume_digits(&mut self) {
        while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            self.position += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            self.position += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.source.as_bytes().get(self.position).copied()
    }

    fn consume(&mut self, byte: u8) -> bool {
        if self.peek() == Some(byte) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, byte: u8, message: &str) -> result::Result<()> {
        if self.consume(byte) {
            Ok(())
        } else {
            Err(self.error(message))
        }
    }

    fn error(&self, message: &str) -> CompileError {
        let prefix = &self.source[..self.position];
        let line = prefix.bytes().filter(|byte| *byte == b'\n').count();
        let column = prefix.rsplit('\n').next().unwrap_or("").len();
        CompileError::Parser(
            Some(Location::new(line, column)),
            message.into(),
            self.source[self.position..].chars().take(10).collect(),
        )
    }
}
