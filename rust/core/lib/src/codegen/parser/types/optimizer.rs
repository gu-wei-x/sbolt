use crate::codegen::CompilerOptions;
use crate::codegen::parser::{Token, tokenizer};

pub(in crate::codegen) trait Optimizer {
    fn accept(&self, token: &Token) -> bool;
}

pub(in crate::codegen) struct HtmlOptimizer<'a> {
    compiler_options: &'a CompilerOptions,
}

impl<'a> HtmlOptimizer<'a> {
    pub(in crate::codegen) fn new(compiler_options: &'a CompilerOptions) -> Self {
        Self {
            compiler_options: compiler_options,
        }
    }
}

impl<'a> Optimizer for HtmlOptimizer<'a> {
    fn accept(&self, token: &Token) -> bool {
        // EOF is special.
        if token.kind() == tokenizer::Kind::EOF {
            return false;
        }

        // todo: cache some tokens for refer to decide whether to accep a token.
        // consume to return all tokens.
        if !self.compiler_options.need_optimization() {
            return true;
        }

        match token.kind() {
            tokenizer::Kind::NEWLINE => false,
            _ => true,
        }
    }
}
