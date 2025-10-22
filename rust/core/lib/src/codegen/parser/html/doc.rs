#![allow(dead_code)]
use crate::codegen::parser::html::{node::Node, parser};

#[derive(Debug, Clone)]
pub(in crate::codegen) struct HtmlDocument {
    nodes: Vec<Node>,
}

impl Default for HtmlDocument {
    fn default() -> Self {
        Self { nodes: vec![] }
    }
}

impl HtmlDocument {
    pub(in crate::codegen) fn to_string(&self) -> String {
        let mut html = String::new();
        for node in &self.nodes {
            html.push_str(&node.to_string());
        }
        html
    }

    pub(in crate::codegen::parser) fn parse<'s>(source: &'s str) -> HtmlDocument {
        let mut state_machine = parser::StateMachine::new(source);
        let dom = state_machine.process();
        dom.clone()
    }

    pub(in crate::codegen::parser::html) fn push_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}
