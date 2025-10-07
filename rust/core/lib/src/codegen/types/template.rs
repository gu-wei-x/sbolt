use crate::codegen::types::block::Block;

pub(in crate::codegen) struct Template<'a> {
    namespace: Option<String>,
    block: Block<'a>,
}

impl<'a> Template<'a> {
    pub(in crate::codegen) fn new(namespace: Option<String>, block: Block<'a>) -> Self {
        Template { namespace, block }
    }

    pub(in crate::codegen) fn namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }

    pub(in crate::codegen) fn block(&self) -> &Block<'a> {
        &self.block
    }
}
