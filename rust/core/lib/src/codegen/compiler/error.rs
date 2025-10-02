use crate::types::error;
use syn::Error;

impl From<std::io::Error> for error::CompileError {
    fn from(io_err: std::io::Error) -> Self {
        error::CompileError::String(io_err.to_string())
    }
}

impl From<Error> for error::CompileError {
    fn from(syn_err: Error) -> Self {
        error::CompileError::String(syn_err.to_string())
    }
}
