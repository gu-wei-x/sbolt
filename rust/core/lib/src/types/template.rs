use crate::types::ViewContext;
use crate::types::Writer;

pub trait Template {
    fn render(&self, context: &mut ViewContext<dyn Writer>);
}

pub trait TemplateResolver {
    fn resolve(&self, name: &str) -> Option<Box<dyn Template>>;
}
