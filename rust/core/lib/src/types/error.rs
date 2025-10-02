use std::ops::Range;
use std::path::PathBuf;

#[derive(Clone)]
pub enum CompileError {
    CodeGen(Range<usize>, String),
    Parser(Option<Range<usize>>, String),
    String(String),
    FileError(PathBuf, Option<Range<usize>>, String),
}

// TODO:
#[allow(dead_code)]
pub enum RuntimeError {
    // could not find view
    NotFound(String),
    // todo: add other types
}
