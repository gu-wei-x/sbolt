use crate::{
    codegen::{
        CompilerOptions,
        compiler::optimizer::{self, Optimizer},
        types::Block,
    },
    types::template,
};

pub(in crate::codegen::compiler) struct CodeGenContext<'a> {
    template_kind: template::Kind,
    options: &'a CompilerOptions,
}

impl<'a> CodeGenContext<'a> {
    pub(in crate::codegen::compiler) fn new(
        template_kind: template::Kind,
        options: &'a CompilerOptions,
    ) -> Self {
        Self {
            template_kind: template_kind,
            options: options,
        }
    }

    pub(in crate::codegen::compiler) fn options(&self) -> &CompilerOptions {
        self.options
    }

    pub(in crate::codegen::compiler) fn create_optimizer(
        &self,
        _block: &Block<'_>,
    ) -> Box<dyn Optimizer + '_> {
        optimizer::create_optimizer(self.template_kind, self.options)
    }
}
