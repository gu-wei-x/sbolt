use crate::codegen::compiler::optimizer::Optimizer;
use crate::types::result;

pub(in crate::codegen::compiler::optimizer) struct DefaultOptimizer;
impl Default for DefaultOptimizer {
    fn default() -> Self {
        Self {}
    }
}

impl Optimizer for DefaultOptimizer {
    fn optimize(&self, source: &str) -> result::Result<String> {
        Ok(source.into())
    }
}
