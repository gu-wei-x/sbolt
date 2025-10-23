use crate::codegen::{CompilerOptions, compiler::optimizer::Optimizer};

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
    fn optimize<'s>(&self, source: &'s str) -> String {
        match self.compiler_options.need_optimization() {
            false => source.into(),
            true => {
                let dom = crate::codegen::parser::html::parse_html(source);
                dom.to_string()
            }
        }
    }
}
