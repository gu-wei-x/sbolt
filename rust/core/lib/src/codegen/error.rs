use crate::codegen::parser;
use crate::types::error;

impl error::Error {
    pub(crate) fn from_parser(token: Option<parser::Token>, str: &str) -> Self {
        error::Error::Parser(token.map(|t| t.range()), str.to_string())
    }
}
