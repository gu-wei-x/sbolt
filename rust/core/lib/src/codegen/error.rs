use crate::codegen::parser;
use crate::codegen::parser::template::Block;
use crate::types::error;
use std::env::VarError;
use std::path::PathBuf;

impl error::CompileError {
    pub(in crate::codegen) fn from_parser(token: Option<parser::Token>, str: &str) -> Self {
        error::CompileError::Parser(token.map(|t| t.range()), str.to_string())
    }

    pub(in crate::codegen) fn from_codegn(block: &Block<'_>, str: &str) -> Self {
        error::CompileError::CodeGen(block.span(), str.to_string())
    }

    pub(in crate::codegen) fn with_file(&self, file: &PathBuf) -> Self {
        match self {
            error::CompileError::CodeGen(range, _) => {
                error::CompileError::FileError(file.clone(), Some(range.clone()), self.to_string())
            }
            error::CompileError::Parser(range, _) => {
                error::CompileError::FileError(file.clone(), range.clone(), self.to_string())
            }
            error::CompileError::String(_) => {
                error::CompileError::FileError(file.clone(), None, self.to_string())
            }
            _ => self.clone(),
        }
    }
}

impl error::CompileError {
    fn format_common(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            error::CompileError::CodeGen(range, str) => {
                write!(f, "CodeGen Err({:?}, {:?})", range, str)
            }
            error::CompileError::FileError(path, range, str) => {
                // todo: format to build error which is same as rustc for ide to highlight issue in related file.
                let mut err_struct = f.debug_struct("Error");
                err_struct
                    .field("file", path)
                    //.field("kind", [cg|generic|parser])
                    .field("postion", range)
                    .field("detail", &str)
                    .finish()
            }
            error::CompileError::Parser(range, str) => {
                write!(f, "Parser Err({:?}, {:?})", range, str)
            }
            error::CompileError::String(msg) => {
                write!(f, "Err:({})", msg)
            }
        }
    }
}

// impl pub traits.
impl std::fmt::Debug for error::CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_common(f)
    }
}

impl std::fmt::Display for error::CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_common(f)
    }
}

impl From<String> for error::CompileError {
    fn from(value: String) -> Self {
        error::CompileError::String(value)
    }
}

impl From<Option<String>> for error::CompileError {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => error::CompileError::String(v),
            None => error::CompileError::String("Unknown error".to_owned()),
        }
    }
}

impl From<&str> for error::CompileError {
    fn from(value: &str) -> Self {
        error::CompileError::String(value.to_owned())
    }
}

impl From<VarError> for error::CompileError {
    fn from(var_err: VarError) -> Self {
        error::CompileError::String(var_err.to_string())
    }
}
