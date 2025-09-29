use crate::types::Writer;

pub trait Template {
    fn name() -> String;
    fn layout() -> Option<String>;
    fn render(&self, output: &mut impl Writer);
    fn get_data<D: Send + Sync + 'static>(&self, key: &str) -> Option<&D>;
}
