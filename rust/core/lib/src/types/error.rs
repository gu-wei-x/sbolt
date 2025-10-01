use std::fmt::Debug;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    CodeGen(String),
    Parser(Option<Range<usize>>, String),
    String(String),
}

impl Error {
    pub(crate) fn from_str(str: &str) -> Self {
        Error::String(str.to_owned())
    }
}

// TODO: how to associate with source error?
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CodeGen(msg) => {
                write!(f, "CodeGen Err({:?})", msg)
            }

            Error::Parser(range, str) => {
                write!(f, "Parser Err({:?}, {:?})", range, str)
            }

            Error::String(msg) => {
                write!(f, "Err:({})", msg)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::String(value)
    }
}

impl From<Option<String>> for Error {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => Error::String(v),
            None => Error::String("Unknown error".to_owned()),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::String(value.to_owned())
    }
}
