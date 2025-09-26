use crate::types::{Context, Writer};

pub trait Template {
    fn name() -> &'static str
    where
        Self: Sized;
    fn render(&self, context: impl Context, output: &mut impl Writer);
}
