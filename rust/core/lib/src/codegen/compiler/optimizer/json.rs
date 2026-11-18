use crate::{
    codegen::{CompilerOptions, compiler::optimizer::Optimizer, parser::json::parse_json},
    types::result,
};

pub(in crate::codegen::compiler::optimizer) struct JsonOptimizer<'a> {
    compiler_options: &'a CompilerOptions,
}

impl<'a> JsonOptimizer<'a> {
    pub(in crate::codegen::compiler::optimizer) fn new(
        compiler_options: &'a CompilerOptions,
    ) -> Self {
        Self { compiler_options }
    }
}

impl Optimizer for JsonOptimizer<'_> {
    fn optimize(&self, source: &str) -> result::Result<String> {
        if self.compiler_options.need_optimization() {
            Ok(parse_json(source)?.to_compact_string())
        } else {
            Ok(source.into())
        }
    }
}
