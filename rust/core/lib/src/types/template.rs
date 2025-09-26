use crate::types::{Context, Writer};

pub trait Template {
    fn name() -> String;
    fn layout() -> Option<String>;
    fn render(&self, context: impl Context, output: &mut impl Writer);
}
