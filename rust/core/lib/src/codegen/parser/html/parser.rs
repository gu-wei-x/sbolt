use crate::codegen::parser::{
    html::{
        doc::HtmlDocument,
        node::{Node, NodeKind},
    },
    tokenizer::{self, Kind, TokenStream, Tokenizer},
};
use winnow::stream::{Stream, TokenSlice};

#[derive(Clone, Copy, Debug)]
pub(in crate::codegen::parser::html) enum State {
    INIT,
    TAGOPEN,
    TAGNAME,
    ATTRS,
    ATTRNAME,
    ATTRVAL,
    TEXT,
    TAGCLOSE,
    // for doctype and comment.
    INSTSTART,
    INST,
    DONE,
}

impl Default for State {
    fn default() -> Self {
        State::INIT
    }
}

#[derive(Debug)]
pub(in crate::codegen::parser::html) struct StateMachine<'s> {
    // current dom.
    dom: HtmlDocument,
    // statck of nodes needs to be parsed.
    nodes: Vec<Node>,
    source: &'s str,
    state: State,
    current_attr_name: String,
}

impl<'s> StateMachine<'s> {
    pub(in crate::codegen::parser::html) fn new(source: &'s str) -> Self {
        Self {
            dom: HtmlDocument::default(),
            nodes: vec![],
            source: source,
            state: State::INIT,
            current_attr_name: "".into(),
        }
    }

    pub(in crate::codegen::parser::html) fn process(&mut self) -> &HtmlDocument {
        let tokenizer = Tokenizer::new(self.source);
        let tokens = tokenizer.into_vec();
        let mut token_stream = TokenSlice::new(&tokens);
        while let Some(_) = token_stream.peek_token() {
            match self.state {
                State::INIT => self.process_with_init(&mut token_stream),
                State::TAGOPEN => self.process_with_tag_open(&mut token_stream),
                State::TAGNAME => self.process_with_tag_name(&mut token_stream),
                State::ATTRS => self.process_with_attributes(&mut token_stream),
                State::ATTRNAME => self.process_with_attr_name(&mut token_stream),
                State::ATTRVAL => self.process_with_attr_value(&mut token_stream),
                State::TEXT => self.process_with_text(&mut token_stream),
                State::TAGCLOSE => self.process_with_tag_close(&mut token_stream),
                State::INSTSTART => self.process_with_instruction_start(&mut token_stream),
                State::INST => self.process_with_instruction(&mut token_stream),
                State::DONE => {
                    self.transit_to(State::DONE);
                    break;
                }
            }
        }

        &self.dom
    }
}

impl<'s> StateMachine<'s> {
    fn process_with_init(&mut self, token_stream: &mut TokenStream) {
        // we know there is valid token;
        let token = token_stream.peek_token().unwrap();
        match token.kind() {
            tokenizer::Kind::LESSTHAN => {
                // consmue '<'
                token_stream.next_token();
                self.transit_to(State::TAGOPEN);
            }
            tokenizer::Kind::EOF => {
                self.transit_to(State::DONE);
            }
            _ => {
                self.transit_to(State::TEXT);
            }
        }
    }

    fn process_with_tag_open(&mut self, token_stream: &mut TokenStream) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::EXPRESSION => {
                    self.transit_to(State::TAGNAME);
                }
                tokenizer::Kind::EXCLAMATION => {
                    // doctype or comment
                    token_stream.next_token();
                    self.transit_to(State::INSTSTART);
                }
                tokenizer::Kind::SLASH => {
                    token_stream.next_token();
                    self.transit_to(State::TAGCLOSE);
                }
                tokenizer::Kind::EOF => {
                    self.transit_to(State::DONE);
                }
                _ => {
                    token_stream.next_token();
                    self.transit_to(State::TAGOPEN);
                }
            }
        } else {
            self.transit_to(State::DONE);
        }
    }

    fn process_with_tag_name(&mut self, token_stream: &mut TokenStream) {
        // here we know there is valid token.
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::EXPRESSION => {
                    // todo: need to handle doctype
                    // save tag name if not quoated.
                    let tag_name = &self.source[token.range()];
                    let node = Node::new_element(tag_name);
                    self.nodes.push(node);
                    token_stream.next_token();
                    self.transit_to(State::ATTRS);
                }
                tokenizer::Kind::GREATTHAN => {
                    token_stream.next_token();
                    self.transit_to(State::INIT);
                }
                tokenizer::Kind::EOF => {
                    self.transit_to(State::DONE);
                }
                _ => {
                    token_stream.next_token();
                    self.transit_to(State::TAGNAME);
                }
            }
        } else {
            self.transit_to(State::DONE);
        }
    }

    fn process_with_attributes(&mut self, token_stream: &mut TokenStream) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::EXPRESSION => {
                    // save att name if not quoated.
                    self.transit_to(State::ATTRNAME)
                }
                tokenizer::Kind::GREATTHAN => {
                    token_stream.next_token();
                    self.transit_to(State::INIT);
                }
                tokenizer::Kind::EOF => {
                    self.transit_to(State::DONE);
                }
                _ => {
                    token_stream.next_token();
                    self.transit_to(State::ATTRS);
                }
            }
        } else {
            self.transit_to(State::DONE);
        }
    }

    fn process_with_attr_name(&mut self, token_stream: &mut TokenStream) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::EXPRESSION => {
                    token_stream.next_token();
                    let attr_name = &self.source[token.range()];
                    self.current_attr_name.push_str(attr_name);
                }
                tokenizer::Kind::EQUALS => {
                    token_stream.next_token();
                    self.transit_to(State::ATTRVAL);
                }
                tokenizer::Kind::GREATTHAN => {
                    token_stream.next_token();
                    if !self.current_attr_name.is_empty() {
                        let node = self.nodes.last_mut().unwrap();
                        node.push_attr(&self.current_attr_name, "");
                        println!("{node:#?}: {}", self.current_attr_name);
                        self.current_attr_name = "".into();
                    }

                    self.transit_to(State::INIT);
                }
                tokenizer::Kind::EOF => {
                    self.transit_to(State::DONE);
                }
                tokenizer::Kind::WHITESPACE | tokenizer::Kind::NEWLINE => {
                    token_stream.next_token();
                    if !self.current_attr_name.is_empty() {
                        self.nodes
                            .last_mut()
                            .unwrap()
                            .push_attr(&self.current_attr_name, "");
                        self.current_attr_name = "".into();
                    }
                }
                _ => {
                    let attr_name = &self.source[token.range()];
                    self.current_attr_name.push_str(attr_name);
                    token_stream.next_token();
                }
            }
        } else {
            self.transit_to(State::DONE);
        }
    }

    fn process_with_attr_value(&mut self, token_stream: &mut TokenStream) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::DQMARK => {
                    let attr_value = get_possible_quotated_string_from_stream(
                        self.source,
                        token_stream,
                        tokenizer::Kind::DQMARK,
                    );
                    self.nodes
                        .last_mut()
                        .unwrap()
                        .push_attr(&self.current_attr_name, &attr_value);
                    self.current_attr_name = "".into();
                    self.transit_to(State::ATTRNAME);
                }
                tokenizer::Kind::SQMAERK => {
                    let attr_value = get_possible_quotated_string_from_stream(
                        self.source,
                        token_stream,
                        tokenizer::Kind::SQMAERK,
                    );
                    self.nodes
                        .last_mut()
                        .unwrap()
                        .push_attr(&self.current_attr_name, &attr_value);
                    self.transit_to(State::ATTRNAME);
                }
                tokenizer::Kind::GREATTHAN => {
                    token_stream.next_token();
                    self.transit_to(State::INIT);
                }
                tokenizer::Kind::EOF => {
                    self.transit_to(State::DONE);
                }
                _ => {
                    token_stream.next_token();
                    self.transit_to(State::ATTRNAME);
                }
            }
        } else {
            self.transit_to(State::DONE);
        }
    }

    fn process_with_text(&mut self, token_stream: &mut TokenStream) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::LESSTHAN => {
                    // consmue '<'
                    token_stream.next_token();
                    self.transit_to(State::TAGOPEN);
                }
                tokenizer::Kind::EOF => {
                    self.transit_to(State::DONE);
                }
                _ => {
                    // save text.
                    token_stream.next_token();
                    let part = &self.source[token.range()];
                    self.nodes.last_mut().unwrap().push_text(&part);
                }
            }
        } else {
            self.transit_to(State::DONE);
        }
    }

    fn process_with_tag_close(&mut self, token_stream: &mut TokenStream) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::EXPRESSION => {
                    // tag name.
                    token_stream.next_token();
                    let tag_name = &self.source[token.range()];
                    let node = Node::new_close_element(tag_name);
                    self.nodes.push(node);
                    self.transit_to(State::TAGCLOSE);
                }
                tokenizer::Kind::GREATTHAN => {
                    // closed
                    token_stream.next_token();
                    self.transit_to(State::INIT);
                }
                tokenizer::Kind::EOF => {
                    self.transit_to(State::DONE);
                }
                _ => {
                    token_stream.next_token();
                    self.transit_to(State::TAGCLOSE);
                }
            }
        } else {
            self.transit_to(State::DONE);
        }
    }

    fn process_with_instruction_start(&mut self, token_stream: &mut TokenStream) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::EXPRESSION => {
                    // doctype
                    token_stream.next_token();
                    let tag_name = &self.source[token.range()];
                    let node = Node::new_element(tag_name);
                    self.nodes.push(node);
                    self.transit_to(State::INST);
                }
                tokenizer::Kind::HYPHEN => {
                    // comment.
                    let node = Node::new_comment();
                    self.nodes.push(node);
                    self.transit_to(State::INST);
                }
                tokenizer::Kind::GREATTHAN => {
                    // closed
                    token_stream.next_token();
                    self.transit_to(State::INIT);
                }
                tokenizer::Kind::EOF => {
                    self.transit_to(State::DONE);
                }
                _ => {
                    // error here: but this is not strict.
                    token_stream.next_token();
                }
            }
        } else {
            self.transit_to(State::DONE);
        }
    }

    fn process_with_instruction(&mut self, token_stream: &mut TokenStream) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::GREATTHAN => {
                    // closed
                    token_stream.next_token();
                    self.transit_to(State::INIT);
                }
                tokenizer::Kind::EOF => {
                    self.transit_to(State::DONE);
                }
                _ => {
                    token_stream.next_token();
                    let text = &self.source[token.range()];
                    self.nodes.last_mut().unwrap().push_text(text);
                }
            }
        } else {
            self.transit_to(State::DONE);
        }
    }

    fn transit_to(&mut self, state: State) {
        match state {
            State::DONE => {
                // pop all nodes and added to dom.
                self.unwind_if_possible();
                self.nodes.reverse();
                while let Some(node) = self.nodes.pop() {
                    self.dom.push_node(node);
                }
            }
            State::INIT => {
                // a node closed
                if let Some(node) = self.nodes.last_mut() {
                    match node.kind() {
                        NodeKind::KELEMENT(tag_name) => {
                            // check whether current is self-closed.
                            // remove from stack and push it to dom.
                            node.set_wellformed();
                            let tag_name = &tag_name.to_lowercase();
                            let self_close_tags = [
                                "meta".to_string(),
                                "link".to_string(),
                                "img".to_string(),
                                "hr".to_string(),
                                "br".to_string(),
                            ];
                            if node.is_closed() {
                                let current_node = self.nodes.pop().unwrap();
                                if let Some(parent_node) = self.nodes.last_mut() {
                                    parent_node.push_node(current_node);
                                } else {
                                    self.dom.push_node(current_node);
                                }
                            } else if self_close_tags.contains(tag_name) {
                                // let next state to unwind.
                                node.close();
                            }
                        }
                        NodeKind::KCELEMENT(c_name) => {
                            node.set_wellformed();
                            let close_node = self.nodes.pop().unwrap();
                            if let Some(parent_node) = self.nodes.last_mut()
                                && matches!(parent_node.kind(), NodeKind::KELEMENT(_))
                            {
                                match parent_node.kind() {
                                    NodeKind::KELEMENT(p_name) if c_name == p_name => {
                                        parent_node.close();

                                        // add parent to dom tree.
                                        let current_node = self.nodes.pop().unwrap();
                                        if let Some(parent_node) = self.nodes.last_mut() {
                                            parent_node.push_node(current_node);
                                        } else {
                                            self.dom.push_node(current_node);
                                        }
                                    }
                                    _ => {
                                        // doen't match: <div>test</a>
                                        // ignore.
                                        return;
                                    }
                                }
                            } else {
                                // </a>test -- valid and let browser to decide.
                                self.dom.push_node(close_node);
                            }
                        }
                        NodeKind::KTEXT => {
                            let current_node = self.nodes.pop().unwrap();
                            if let Some(parent_node) = self.nodes.last_mut() {
                                parent_node.push_node(current_node);
                            } else {
                                self.dom.push_node(current_node);
                            }
                        }
                        NodeKind::KCOMMENT => {
                            node.set_wellformed();
                            let current_node = self.nodes.pop().unwrap();
                            if let Some(parent_node) = self.nodes.last_mut() {
                                parent_node.push_node(current_node);
                            } else {
                                self.dom.push_node(current_node);
                            }
                        }
                    }
                }
            }
            State::TAGOPEN => {
                self.unwind_if_possible();
                if let Some(node) = self.nodes.last_mut() {
                    match node.kind() {
                        NodeKind::KTEXT => {
                            // don't need to close add it to dom.
                            let current_node = self.nodes.pop().unwrap();
                            if let Some(parent_node) = self.nodes.last_mut() {
                                parent_node.push_node(current_node);
                            } else {
                                self.dom.push_node(current_node);
                            }
                        }
                        _ => {
                            // do nth.
                        }
                    }
                }
            }
            State::TEXT => {
                self.unwind_if_possible();
                let node = Node::new_text();
                self.nodes.push(node);
            }
            _ => {
                self.unwind_if_possible();
            }
        }

        self.state = state;
    }

    fn unwind_if_possible(&mut self) {
        if let Some(node) = self.nodes.last_mut() {
            match node.kind() {
                NodeKind::KELEMENT(_) => {
                    if node.is_closed() {
                        let current_node = self.nodes.pop().unwrap();
                        if let Some(parent_node) = self.nodes.last_mut() {
                            parent_node.push_node(current_node);
                        } else {
                            self.dom.push_node(current_node);
                        }
                    }
                }
                _ => {
                    // do nth.
                }
            }
        }
    }
}

// consume token_stream to get quotated string.
fn get_possible_quotated_string_from_stream(
    source: &str,
    token_stream: &mut TokenStream,
    mark_kind: Kind,
) -> String {
    let mut content = String::new();
    if let Some(token) = token_stream.peek_token() {
        if token.kind() != mark_kind {
            return content;
        }

        token_stream.next_token();
        while let Some(token) = token_stream.peek_token() {
            if token.kind() == mark_kind {
                token_stream.next_token();
                break;
            } else if token.kind() == tokenizer::Kind::LESSTHAN {
                // don't consume.
                break;
            } else if token.kind() != tokenizer::Kind::NEWLINE {
                // todo: how to deal with: attr="v1    v2";
                let part = &source[token.range()];
                content.push_str(part);
            }

            token_stream.next_token();
        }
    }
    content
}

#[allow(dead_code)]
fn is_valid_name_token(_token_kind: tokenizer::Kind) -> bool {
    true
}

#[allow(dead_code)]
fn is_valid_atti_name_token(_token_kind: tokenizer::Kind) -> bool {
    // start-with: data-
    // lowercase
    // a-z, -, :, ., _
    true
}
