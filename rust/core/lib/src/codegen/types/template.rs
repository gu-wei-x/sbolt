use crate::codegen::types::block::Block;
use crate::types::template::Kind;

pub(in crate::codegen) struct Template<'a> {
    namespace: Option<String>,
    block: Block<'a>,
    kind: Kind,
}

impl<'a> Template<'a> {
    pub(in crate::codegen) fn new(namespace: Option<String>, block: Block<'a>, kind: Kind) -> Self {
        Template {
            namespace,
            block,
            kind,
        }
    }

    pub(in crate::codegen) fn namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }

    pub(in crate::codegen) fn block(&self) -> &Block<'a> {
        &self.block
    }

    pub(in crate::codegen) fn kind(&self) -> Kind {
        self.kind
    }
}
