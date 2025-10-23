use indexmap::map;

#[derive(Clone, Debug)]
pub(in crate::codegen::parser::html) enum NodeKind {
    KELEMENT(String),
    // </div>, see statemachine why this is needed.
    KCELEMENT(String),
    KTEXT,
    KCOMMENT,
}

#[derive(Debug, Clone)]
pub(in crate::codegen::parser::html) struct Node {
    kind: NodeKind,
    attributes: map::IndexMap<String, String>,
    children: Vec<Node>,
    is_wellformed: bool,
    is_closed: bool,
    text: String,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            kind: NodeKind::KELEMENT("".into()),
            attributes: map::IndexMap::new(),
            children: vec![],
            is_wellformed: false,
            is_closed: false,
            text: "".into(),
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

    pub(in crate::codegen::parser::html) fn is_pre(&self) -> bool {
        let pre_tag_name = "pre";
        match self.kind() {
            NodeKind::KELEMENT(tag_name) | NodeKind::KCELEMENT(tag_name)
                if tag_name.to_lowercase() == pre_tag_name =>
            {
                true
            }
            _ => false,
        }
    }

    pub(in crate::codegen::parser::html) fn is_doctype(&self) -> bool {
        let doctype_tag_name = "doctype";
        match self.kind() {
            NodeKind::KELEMENT(tag_name) if tag_name.to_lowercase() == doctype_tag_name => true,
            _ => false,
        }
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
            text: "".into(),
        }
    }

    pub(in crate::codegen::parser::html) fn new_text() -> Self {
        Node {
            kind: NodeKind::KTEXT,
            attributes: map::IndexMap::new(),
            children: vec![],
            is_wellformed: true,
            is_closed: true,
            text: "".into(),
        }
    }

    pub(in crate::codegen::parser::html) fn new_comment() -> Self {
        Node {
            kind: NodeKind::KCOMMENT,
            attributes: map::IndexMap::new(),
            children: vec![],
            is_wellformed: false,
            is_closed: false,
            text: "".into(),
        }
    }

    pub(in crate::codegen::parser::html) fn push_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    pub(in crate::codegen::parser::html) fn new_close_element(name: &str) -> Self {
        Node {
            kind: NodeKind::KCELEMENT(name.into()),
            attributes: map::IndexMap::new(),
            children: vec![],
            is_wellformed: false,
            is_closed: false,
            text: "".into(),
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

    pub(in crate::codegen::parser::html) fn to_string(&self, parent: Option<&Node>) -> String {
        let mut content = String::new();
        match &self.kind {
            NodeKind::KTEXT => match parent {
                Some(p) => match p.kind() {
                    NodeKind::KELEMENT(tag_name) if tag_name.to_lowercase() == "pre" => {
                        content.push_str(&self.text);
                    }
                    _ => {
                        content.push_str(&self.text.trim());
                    }
                },
                _ => content.push_str(&self.text),
            },
            NodeKind::KCOMMENT => {
                // no string for comment.
                // content.push_str(&format!("<!{}>", self.text));
            }
            NodeKind::KELEMENT(tag_name) => {
                if self.is_doctype() {
                    content.push_str(&format!("<!{}", tag_name));
                } else {
                    content.push_str(&format!("<{}", tag_name));
                }
                for (attr_name, attr_value) in &self.attributes {
                    content.push_str(&format!(" {}=\"{}\"", attr_name, attr_value));
                }

                if self.is_doctype() {
                    content.push_str(&self.text);
                }

                // children.
                let mut c_content = String::new();
                for node in &self.children {
                    let node_content = node.to_string(Some(self));
                    c_content.push_str(&node_content);
                }

                // whether wellformed: impossible to have child if not.
                if self.is_wellformed() {
                    // closed.
                    match c_content.is_empty() {
                        true if self.is_closed() => {
                            content.push_str("/>");
                        }
                        _ => {
                            content.push_str(">");
                            content.push_str(&c_content);
                            if self.is_closed() {
                                content.push_str(&format!("</{}>", tag_name));
                            }
                        }
                    }
                }
            }
            NodeKind::KCELEMENT(tag_name) => {
                content.push_str(&format!("</{}>", tag_name));
            }
        }
        content
    }
}
