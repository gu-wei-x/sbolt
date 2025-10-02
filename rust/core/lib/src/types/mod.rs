mod context;
mod data_store;
mod macros;
mod template;
mod writer;

#[cfg(test)]
mod tests;

pub use context::*;
pub use data_store::*;
pub use template::Template;
pub use writer::*;

pub mod error {
    use std::path::PathBuf;

    #[derive(Clone)]
    pub enum CompileError {
        CodeGen(
            super::Location,
            /*summary*/ String,
            /*detail*/ String,
        ),
        Parser(
            Option<super::Location>,
            /*summary*/ String,
            /*detail*/ String,
        ),
        String(String),
        FileError(
            PathBuf,
            Option<super::Location>,
            /*summary*/ String,
            /*detail*/ Option<String>,
        ),
    }

    // TODO:
    #[allow(dead_code)]
    pub enum RuntimeError {
        // could not find view
        NotFound(/*summary*/ String, /*detail*/ String),
        // todo: add other types
    }
}

pub mod result {
    pub type Result<T> = core::result::Result<T, super::error::CompileError>;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}
