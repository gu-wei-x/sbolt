use crate::codegen::parser::{Token, optimizer::Optimizer, tokenizer::Kind};

pub(in crate::codegen::parser::optimizer) struct DefaultOptimizer;
impl Default for DefaultOptimizer {
    fn default() -> Self {
        Self {}
    }
}

impl Optimizer for DefaultOptimizer {
    fn accept(&mut self, token: &Token) -> bool {
        // EOF is special.
        if token.kind() == Kind::EOF {
            return false;
        }

        return true;
    }
}
