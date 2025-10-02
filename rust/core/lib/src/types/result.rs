use crate::types::error;

pub type Result<T> = core::result::Result<T, error::CompileError>;

impl<T> From<error::CompileError> for Result<T> {
    fn from(error: error::CompileError) -> Self {
        Err(error)
    }
}
