use crate::{
    codegen::parser::html::{
        node::{Node, NodeKind},
        parser,
    },
    types::result,
};

#[derive(Debug)]
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
    // Try to generate an optimized html string with limited context.
    // Here dom is not a full/wellformed dom, not even from root but a fragment with unknow context to do a perfect optimization.
    pub(in crate::codegen) fn to_string(&self) -> String {
        let count = self.nodes.len();
        if count == 0 {
            return "".into();
        } else if count == 1 {
            // no context with only one node.
            return self.nodes[0].to_string(None);
        }

        // at least 2 node.
        // need to check the node context when generating the html string from node.
        // (unknown)TEXTNODE - NODE - TEXTNODE(unknown) => left - NODE - right
        // NODE:= ELMENT|ECLEMENT|COMENT
        // ELEMENT:
        //    left: could trim end as end context is NODE which spaces/lf have no meaning
        //    right: could trim start if ELEMENT is not <pre>
        //
        // ECLEMENT:
        //    left: could trim end if ECLEMENT is not </pre>
        //    right: could trim left as start context is node.
        //
        // COMMENT: todo later
        //    left: trim end
        //    right: trim start.
        // zip the nodes pairs.
        let result = self
            .nodes
            .iter()
            .zip(self.nodes.iter().skip(1))
            .collect::<Vec<_>>();
        let last_index = result.len() - 1;
        let mut html = String::new();
        for (index, (c, n)) in result.iter().enumerate() {
            if index == last_index {
                match (c.kind(), n.kind()) {
                    (NodeKind::KELEMENT(_), NodeKind::KTEXT) => {
                        html.push_str(&c.to_string(None));
                        let content = n.to_string(None);
                        let content = if c.is_pre() {
                            &content
                        } else {
                            content.trim_start()
                        };

                        if !content.is_empty() {
                            html.push_str(content);
                        }
                    }
                    (NodeKind::KCELEMENT(_), NodeKind::KTEXT) => {
                        html.push_str(&c.to_string(None));
                        let content = n.to_string(None);
                        let content = content.trim_start();
                        if !content.is_empty() {
                            html.push_str(content);
                        }
                    }
                    (NodeKind::KTEXT, NodeKind::KELEMENT(_)) => {
                        let content = c.to_string(None);
                        let content = content.trim_end();
                        if !content.is_empty() {
                            html.push_str(content);
                        }
                        html.push_str(&n.to_string(None));
                    }
                    (NodeKind::KTEXT, NodeKind::KCELEMENT(_)) => {
                        let content = c.to_string(None);
                        let content = if c.is_pre() {
                            &content
                        } else {
                            content.trim_end()
                        };
                        if !content.is_empty() {
                            html.push_str(content);
                        }
                        html.push_str(&n.to_string(None));
                    }
                    (NodeKind::KTEXT, _) => {
                        let content = c.to_string(None);
                        let content = content.trim_end();
                        if !content.is_empty() {
                            html.push_str(content);
                        }
                        html.push_str(&n.to_string(None));
                    }
                    (_, NodeKind::KTEXT) => {
                        html.push_str(&c.to_string(None));
                        let content = n.to_string(None);
                        let content = content.trim_start();
                        if !content.is_empty() {
                            html.push_str(content);
                        }
                    }
                    (_, _) => {
                        html.push_str(&c.to_string(None));
                        html.push_str(&n.to_string(None));
                    }
                }
            } else {
                match (c.kind(), n.kind()) {
                    (NodeKind::KTEXT, NodeKind::KELEMENT(_)) => {
                        let content = c.to_string(None);
                        let content = content.trim_end();
                        if !content.is_empty() {
                            html.push_str(content);
                        }
                    }

                    (NodeKind::KTEXT, NodeKind::KCELEMENT(_)) => {
                        let content = c.to_string(None);
                        let content = if c.is_pre() {
                            &content
                        } else {
                            content.trim_end()
                        };
                        if !content.is_empty() {
                            html.push_str(content);
                        }
                    }
                    (NodeKind::KTEXT, _) => {
                        let content = c.to_string(None);
                        let content = content.trim_end();
                        if !content.is_empty() {
                            html.push_str(content);
                        }
                    }
                    (_, _) => {
                        html.push_str(&c.to_string(None));
                    }
                }
            }
        }

        html
    }

    pub(in crate::codegen::parser) fn parse<'s>(source: &'s str) -> result::Result<HtmlDocument> {
        let mut state_machine = parser::StateMachine::new(source);
        state_machine.process()
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
