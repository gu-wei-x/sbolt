use crate::{
    codegen::{CompilerOptions, StaticOutputStats, compiler::optimizer},
    types::{result, template},
};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub(in crate::codegen::compiler) struct CodeGenContext<'a> {
    template_kind: template::Kind,
    options: &'a CompilerOptions,
    static_context_known: Cell<bool>,
    stats: Rc<RefCell<StaticOutputStats>>,
}

impl<'a> CodeGenContext<'a> {
    pub(in crate::codegen::compiler) fn new(
        template_kind: template::Kind,
        options: &'a CompilerOptions,
    ) -> Self {
        Self {
            template_kind: template_kind,
            options: options,
            static_context_known: Cell::new(true),
            stats: Rc::default(),
        }
    }

    pub(in crate::codegen::compiler) fn options(&self) -> &CompilerOptions {
        self.options
    }

    pub(in crate::codegen::compiler) fn optimize_static_output(&self) -> bool {
        self.options.need_optimization() && self.template_kind != template::Kind::KTEXT
    }

    pub(in crate::codegen::compiler) fn invalidate_static_context(&self) {
        self.static_context_known.set(false);
    }

    pub(in crate::codegen::compiler) fn prepare_root(&self, fully_static: bool) {
        if self.template_kind == template::Kind::KJSON && !fully_static {
            self.invalidate_static_context();
        }
    }

    pub(in crate::codegen::compiler) fn section_context(&self) -> Self {
        Self {
            template_kind: self.template_kind,
            options: self.options,
            static_context_known: Cell::new(false),
            stats: Rc::clone(&self.stats),
        }
    }

    pub(in crate::codegen::compiler) fn static_content(
        &self,
        content: &str,
        input_writes: usize,
    ) -> result::Result<Option<String>> {
        let optimize = self.optimize_static_output();
        let unknown_context = optimize && !self.static_context_known.get();
        let output = if optimize && !unknown_context {
            optimizer::create_optimizer(self.template_kind, self.options).optimize(content)?
        } else {
            content.into()
        };
        let emitted = !optimize || !output.is_empty();
        self.stats
            .borrow_mut()
            .record(content, input_writes, &output, emitted, unknown_context);
        Ok(emitted.then_some(output))
    }

    pub(in crate::codegen::compiler) fn static_output_stats(&self) -> StaticOutputStats {
        self.stats.borrow().clone()
    }
}
