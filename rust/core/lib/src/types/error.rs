use crate::codegen::parser;
use std::fmt::Debug;

#[allow(private_interfaces)]
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    CodeGen(String),
    String(String),
    Parser(Option<parser::Token>, String),
}

impl Error {
    pub(crate) fn from_str(str: &str) -> Self {
        Error::String(str.to_owned())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CodeGen(msg) => {
                write!(f, "CodeGen Err({:?})", msg)
            }

            Error::String(msg) => {
                write!(f, "Err:({})", msg)
            }

            Error::Parser(token, str) => {
                write!(f, "Parser Err({:?}, {:?})", token, str)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
