use crate::types::Context;
use crate::types::result;

/// The template representation to render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
