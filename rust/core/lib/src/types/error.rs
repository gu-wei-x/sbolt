use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    CodeGen(String),
    String(String),
    Parser(String),
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

            Error::Parser(token) => {
                write!(f, "Paser Err({:?})", token)
            }
        }
    }
}
