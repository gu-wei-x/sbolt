use crate::types::Context;

pub trait Template {
    fn name() -> &'static str
    where
        Self: Sized;

    fn render(&self, context: &mut impl Context);
}
