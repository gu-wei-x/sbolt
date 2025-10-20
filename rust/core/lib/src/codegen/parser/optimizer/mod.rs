mod default;
mod html;

#[cfg(test)]
mod tests;

use crate::codegen::CompilerOptions;
use crate::codegen::parser::Token;
use crate::types::template;

pub(in crate::codegen::parser) trait Optimizer {
    fn accept<'s>(&mut self, source: &'s str, token: &Token) -> bool;
}

pub(in crate::codegen::parser) fn create_optimizer<'a>(
    template_kind: template::Kind,
    options: &'a CompilerOptions,
) -> Box<dyn Optimizer + 'a> {
    match template_kind {
        template::Kind::KHTML => Box::new(html::HtmlOptimizer::new(options)),
        _ => Box::new(default::DefaultOptimizer::default()),
    }
}
