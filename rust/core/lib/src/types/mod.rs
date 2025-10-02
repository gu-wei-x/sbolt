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
        CodeGen((usize, usize), String),
        Parser(Option<(usize, usize)>, String),
        String(String),
        FileError(PathBuf, Option<(usize, usize)>, String),
    }

    // TODO:
    #[allow(dead_code)]
    pub enum RuntimeError {
        // could not find view
        NotFound(String),
        // todo: add other types
    }
}

pub mod result {
    pub type Result<T> = core::result::Result<T, super::error::CompileError>;
}
