use crate::types::error;

impl error::Error {
    pub(crate) fn from_parser(error: &str) -> Self {
        error::Error::Parser(error.to_string())
    }
}
