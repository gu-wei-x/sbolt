#![allow(dead_code)]
use crate::codegen::parser::html::{
    node::{Node, NodeKind},
    parser,
};

#[derive(Debug, Clone)]
pub(in crate::codegen) struct HtmlDocument {
    nodes: Vec<Node>,
    // is closed fragment.
    is_closed: bool,
}

impl Default for HtmlDocument {
    fn default() -> Self {
        Self {
            nodes: vec![],
            is_closed: true,
        }
    }
}

impl HtmlDocument {
    pub(in crate::codegen) fn to_string(&self) -> String {
        let mut html = String::new();
        // here: context is unknow but only fragments.
        // need to handle
        // text(this we know it's not in pre, could trim end) - node - text(this one we know it's not in pre, could trim start.)
        if self.nodes.len() <= 1 {
            for node in &self.nodes {
                // no context.
                html.push_str(&node.to_string(None));
            }
            return html;
        }

        let result = self
            .nodes
            .iter()
            .zip(self.nodes.iter().skip(1))
            .inspect(|(a, b)| {
                println!("**************");
                println!("a: {:?}\n b: {:?}", a, b);
            })
            .collect::<Vec<_>>();
        let last_index = result.len() - 1;
        for (index, (p, n)) in result.iter().enumerate() {
            if index == last_index {
                match (p.kind(), n.kind()) {
                    (NodeKind::KELEMENT(_) | NodeKind::KCELEMENT(_), NodeKind::KTEXT) => {
                        html.push_str(&p.to_string(None));
                        let content = n.to_string(None);
                        let content = content.trim_start();
                        if !content.is_empty() {
                            html.push_str(content);
                        }
                    }
                    (NodeKind::KTEXT, NodeKind::KELEMENT(_) | NodeKind::KCELEMENT(_)) => {
                        let content = p.to_string(None);
                        let content = content.trim_end();
                        if !content.is_empty() {
                            html.push_str(content);
                        }
                        html.push_str(&n.to_string(None));
                    }
                    (_, _) => {
                        html.push_str(&p.to_string(None));
                        html.push_str(&n.to_string(None));
                    }
                }
            } else {
                match (p.kind(), n.kind()) {
                    (NodeKind::KTEXT, NodeKind::KELEMENT(_) | NodeKind::KCELEMENT(_)) => {
                        let content = p.to_string(None);
                        let content = content.trim_end();
                        if !content.is_empty() {
                            html.push_str(content);
                        }
                    }
                    (_, _) => {
                        // do nth for n, next iteration.
                        html.push_str(&p.to_string(None));
                    }
                }
            }
        }

        html
    }

    pub(in crate::codegen::parser) fn parse<'s>(source: &'s str) -> HtmlDocument {
        let mut state_machine = parser::StateMachine::new(source);
        let dom = state_machine.process();
        dom.clone()
    }

    pub(in crate::codegen::parser::html) fn push_node(&mut self, node: Node) {
        if self.is_closed {
            match node.kind() {
                NodeKind::KELEMENT(_) if !node.is_closed() => self.is_closed = false,
                NodeKind::KCELEMENT(_) => self.is_closed = false,
                _ => {}
            }
        }
        self.nodes.push(node);
    }
}
