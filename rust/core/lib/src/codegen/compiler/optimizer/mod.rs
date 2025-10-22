#![allow(dead_code)]
mod default;
mod html;

#[cfg(test)]
mod tests;

use crate::codegen::CompilerOptions;
use crate::types::template;

pub(in crate::codegen::compiler) trait Optimizer {
    fn optimize<'s>(&self, source: &'s str) -> String;
}

pub(in crate::codegen::compiler) fn create_optimizer<'a>(
    template_kind: template::Kind,
    options: &'a CompilerOptions,
) -> Box<dyn Optimizer + 'a> {
    match template_kind {
        template::Kind::KHTML => Box::new(html::HtmlOptimizer::new(options)),
        _ => Box::new(default::DefaultOptimizer::default()),
    }
}
