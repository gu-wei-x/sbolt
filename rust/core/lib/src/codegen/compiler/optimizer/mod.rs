mod default;
mod html;
mod json;

#[cfg(test)]
mod tests;

use crate::codegen::CompilerOptions;
use crate::types::{result, template};

pub(in crate::codegen::compiler) trait Optimizer {
    fn optimize(&self, source: &str) -> result::Result<String>;
}

pub(in crate::codegen::compiler) fn create_optimizer<'a>(
    template_kind: template::Kind,
    options: &'a CompilerOptions,
) -> Box<dyn Optimizer + 'a> {
    match template_kind {
        template::Kind::KHTML => Box::new(html::HtmlOptimizer::new(options)),
        template::Kind::KJSON => Box::new(json::JsonOptimizer::new(options)),
        _ => Box::new(default::DefaultOptimizer::default()),
    }
}
