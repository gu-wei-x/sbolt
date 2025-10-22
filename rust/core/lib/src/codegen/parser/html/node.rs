#![allow(dead_code)]
use crate::codegen::parser::tokenizer::{self, TokenStream, skip_whitespace_and_newline};
use indexmap::map;
use winnow::stream::Stream as _;

#[derive(Clone, Debug)]
pub(in crate::codegen::parser::html) enum NodeKind {
    KELEMENT(String),
    // </div>, see statemachine why this is needed.
    KCELEMENT(String),
    KTEXT(String),
    KCOMMENT(String),
}

#[derive(Debug, Clone)]
pub(in crate::codegen::parser::html) struct Node {
    kind: NodeKind,
    attributes: map::IndexMap<String, String>,
    children: Vec<Node>,
    is_wellformed: bool,
    is_closed: bool,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            kind: NodeKind::KELEMENT("".into()),
            attributes: map::IndexMap::new(),
            children: vec![],
            is_wellformed: false,
            is_closed: false,
        }
    }
}

impl Node {
    pub(in crate::codegen::parser::html) fn kind(&self) -> NodeKind {
        self.kind.clone()
    }

    pub(in crate::codegen::parser::html) fn set_wellformed(&mut self) {
        self.is_wellformed = true;
    }

    pub(in crate::codegen::parser::html) fn is_wellformed(&self) -> bool {
        self.is_wellformed
    }

    pub(in crate::codegen::parser::html) fn close(&mut self) {
        self.is_closed = true;
    }

    pub(in crate::codegen::parser::html) fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub(in crate::codegen::parser::html) fn new_element(name: &str) -> Self {
        Node {
            kind: NodeKind::KELEMENT(name.into()),
            attributes: map::IndexMap::new(),
            children: vec![],
            is_wellformed: false,
            is_closed: false,
        }
    }

    pub(in crate::codegen::parser::html) fn new_text(name: &str) -> Self {
        Node {
            kind: NodeKind::KTEXT(name.into()),
            attributes: map::IndexMap::new(),
            children: vec![],
            is_wellformed: true,
            is_closed: true,
        }
    }

    pub(in crate::codegen::parser::html) fn new_close_element(name: &str) -> Self {
        Node {
            kind: NodeKind::KCELEMENT(name.into()),
            attributes: map::IndexMap::new(),
            children: vec![],
            is_wellformed: false,
            is_closed: false,
        }
    }

    pub(in crate::codegen::parser::html) fn push_attr(
        &mut self,
        attr_name: &str,
        attr_value: &str,
    ) {
        let attr_value = attr_value.trim();
        if !attr_value.is_empty() {
            self.attributes.insert(attr_name.into(), attr_value.into());
        }
    }

    pub(in crate::codegen::parser::html) fn push_node(&mut self, node: Node) {
        self.children.push(node);
    }

    pub(in crate::codegen::parser::html) fn to_string(&self) -> String {
        let mut content = String::new();
        match &self.kind {
            NodeKind::KTEXT(str) => content.push_str(&str),
            NodeKind::KCOMMENT(_str) => {}
            NodeKind::KELEMENT(tag_name) => {
                content.push_str(&format!("<{}", tag_name));
                for (attr_name, attr_value) in &self.attributes {
                    content.push_str(&format!(" {}=\"{}\"", attr_name, attr_value));
                }
                match self.children.is_empty() {
                    true => {
                        content.push_str("/>");
                    }
                    false => {
                        content.push_str(">");
                        for node in &self.children {
                            let node_content = node.to_string();
                            content.push_str(&node_content);
                        }
                        content.push_str(&format!("</{}>", tag_name));
                    }
                }
            }
            NodeKind::KCELEMENT(tag_name) => {
                content.push_str(&format!("</{}>", tag_name));
            }
        }
        content
    }

    pub(in crate::codegen::parser::html) fn from<'s>(
        source: &'s str,
        token_stream: &mut TokenStream,
    ) -> Option<Self> {
        let mut node = Node::default();
        token_stream.next_token();

        // starts with <
        token_stream.next_token();
        skip_whitespace_and_newline(token_stream);
        while let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::EXPRESSION => {
                    let tag_name = &source[token.range()];
                    node = Node::new_element(tag_name);
                    token_stream.next_token();

                    // todo: attributes, text node.
                }
                tokenizer::Kind::SLASH => {
                    // close tag
                    token_stream.next_token();
                }
                tokenizer::Kind::GREATTHAN => {
                    // close tag
                    token_stream.next_token();
                }
                tokenizer::Kind::LESSTHAN => {
                    // start new.
                    let child_node = Node::from(source, token_stream);
                    if child_node.is_some() {
                        node.push_node(child_node.unwrap());
                    }
                }
                _ => {
                    token_stream.next_token();
                }
            }
        }

        Some(node)
    }
}
