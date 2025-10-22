use crate::codegen::compiler::optimizer::Optimizer;

pub(in crate::codegen::compiler::optimizer) struct DefaultOptimizer;
impl Default for DefaultOptimizer {
    fn default() -> Self {
        Self {}
    }
}

impl Optimizer for DefaultOptimizer {
    fn optimize<'s>(&self, source: &'s str) -> String {
        source.into()
    }
}
