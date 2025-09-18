use crate::types::error;

pub type Result<T> = core::result::Result<T, error::Error>;

impl<T> From<error::Error> for Result<T> {
    fn from(error: error::Error) -> Self {
        Err(error)
    }
}
