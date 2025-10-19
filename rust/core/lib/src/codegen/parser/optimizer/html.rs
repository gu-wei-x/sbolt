use crate::codegen::{
    CompilerOptions,
    parser::{Token, optimizer::Optimizer, tokenizer::Kind},
};

pub(in crate::codegen::parser::optimizer) struct HtmlOptimizer<'a> {
    compiler_options: &'a CompilerOptions,
    _is_in_pre: bool,
    _tokens: Vec<Token>,
}

impl<'a> HtmlOptimizer<'a> {
    pub(in crate::codegen::parser::optimizer) fn new(
        compiler_options: &'a CompilerOptions,
    ) -> Self {
        Self {
            compiler_options: compiler_options,
            _is_in_pre: false,
            _tokens: vec![],
        }
    }
}

impl<'a> Optimizer for HtmlOptimizer<'a> {
    fn accept(&mut self, token: &Token) -> bool {
        // EOF is special.
        if token.kind() == Kind::EOF {
            return false;
        }

        if !self.compiler_options.need_optimization() {
            return true;
        }

        // todo: cache some tokens for refer to decide whether to accep a token.
        // consume to return all tokens.
        match token.kind() {
            Kind::LESSTHAN => {
                // open tag
                true
            }
            Kind::GREATTHAN => {
                // close tag
                true
            }
            Kind::EXPRESSION => {
                // tag name or other
                true
            }
            Kind::NEWLINE => false,
            _ => true,
        }
    }
}
