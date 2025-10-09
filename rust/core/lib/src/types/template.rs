use crate::types::Context;
use crate::types::result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    KJSON,
    KHTML,
    KTEXT,
}

pub trait Template {
    fn name() -> String;
    fn kind() -> Kind;
    fn layout() -> Option<String> {
        // default impl.
        None
    }
    fn render(&self, context: &mut impl Context) -> result::RenderResult<String>;
}
