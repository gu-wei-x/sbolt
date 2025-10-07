use crate::types::error;
use crate::types::result;

pub trait Template {
    fn name() -> String;
    fn layout() -> Option<String> {
        // default impl.
        None
    }
    fn get_data<D: Send + Sync + 'static>(&self, key: &str) -> Option<&D>;
    fn render(&self) -> result::RenderResult<String> {
        // default impl.
        Err(error::RuntimeError::view_not_found(&Self::name()))
    }
}
