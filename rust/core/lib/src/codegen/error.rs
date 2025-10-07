use crate::codegen::parser;
use crate::types::{Location, error};
use std::env::VarError;
use std::path::PathBuf;

impl error::CompileError {
    pub(in crate::codegen) fn from_parser<'a>(
        source: &'a str,
        token: Option<parser::Token>,
        summary: &str,
    ) -> Self {
        let detail = match token {
            Some(t) => {
                let start = t.range().start;
                let end = (start + 10).min(source.len());
                source[start..end].to_string()
            }
            None => "".to_string(),
        };

        error::CompileError::Parser(token.map(|t| t.location()), summary.to_string(), detail)
    }

    pub(in crate::codegen) fn with_file(&self, file: &PathBuf) -> Self {
        match self {
            error::CompileError::CodeGen(location, str, detail) => error::CompileError::FileError(
                file.clone(),
                Some(*location),
                str.to_string(),
                Some(detail.to_string()),
            ),
            error::CompileError::Parser(location, str, detail) => error::CompileError::FileError(
                file.clone(),
                location.clone(),
                str.to_string(),
                Some(detail.to_string()),
            ),
            error::CompileError::String(str) => {
                error::CompileError::FileError(file.clone(), None, str.to_string(), None)
            }
            _ => self.clone(),
        }
    }
}

impl error::CompileError {
    fn format(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            error::CompileError::CodeGen(range, str, _) => {
                write!(f, "CodeGen Err({:?}, {:?})", range, str)
            }
            error::CompileError::FileError(path, location, summary, detail) => {
                let mut result = String::new();
                result.push_str(&format!("{summary}\n"));
                let abs_path = match std::path::absolute(path) {
                    Ok(p) => p,
                    Err(_) => path.clone(),
                };

                match location {
                    Some(loc) => {
                        result.push_str(&format!("\t--> {abs_path:?}:{loc}\n"));
                        match detail {
                            Some(content) => {
                                result.push_str("\t|\n");
                                result.push_str(&format!("\t{loc}|\t{content}\n"));
                                result.push_str("\t|");
                            }
                            None => {}
                        }
                    }
                    None => {
                        result.push_str(&format!(" --> {abs_path:?}\n"));
                    }
                }

                write!(f, "{result}")
            }
            error::CompileError::Parser(range, str, _) => {
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
        self.format(f)
    }
}

impl std::fmt::Display for error::CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f)
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

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}
