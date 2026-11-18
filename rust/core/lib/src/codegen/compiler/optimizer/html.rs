use crate::codegen::{CompilerOptions, compiler::optimizer::Optimizer};
use crate::types::result;

pub(in crate::codegen::compiler::optimizer) struct HtmlOptimizer<'a> {
    compiler_options: &'a CompilerOptions,
}

impl<'a> HtmlOptimizer<'a> {
    pub(in crate::codegen::compiler::optimizer) fn new(
        compiler_options: &'a CompilerOptions,
    ) -> Self {
        Self {
            compiler_options: compiler_options,
        }
    }
}

impl<'a> Optimizer for HtmlOptimizer<'a> {
    fn optimize(&self, source: &str) -> result::Result<String> {
        Ok(match self.compiler_options.need_optimization() {
            false => source.into(),
            true => crate::codegen::parser::html::compact_html_whitespace(source),
        })
    }
}
