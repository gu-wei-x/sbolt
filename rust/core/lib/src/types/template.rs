use crate::types::Context;
use crate::types::result;

pub trait Template {
    fn name() -> String;
    fn layout() -> Option<String> {
        // default impl.
        None
    }
    fn render(&self, context: &mut impl Context) -> result::RenderResult<String>;
}
