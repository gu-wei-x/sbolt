mod context;
mod data_store;
mod functions;
mod macros;
pub mod template;
mod writer;

pub use context::*;
pub use data_store::*;
pub use functions::*;
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

    #[derive(Debug)]
    pub enum RuntimeError {
        // could not find view
        NotFound(/*summary*/ String, /*detail*/ String),
        // todo: add other types
    }
}

pub mod result {
    pub type Result<T> = std::result::Result<T, super::error::CompileError>;
    pub type RenderResult<T> = std::result::Result<T, super::error::RuntimeError>;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}
