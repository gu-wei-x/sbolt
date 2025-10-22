#![allow(dead_code)]
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
    DONE,
}

impl Default for State {
    fn default() -> Self {
        State::INIT
    }
}

// The impl is not to parse a welfomed doc but fragments as the whole doc is devided by diffrent type of blocks
// the goal is to do as much as possible to optimize blocks to remove unneeded chars from static cotnent to reduce bits size and ops on str operations.
// "<div att1="a">test " => node[node]
// "<div att1=" => node(not welformed)
// "<div att="a">test</div>" => node(welformed)

#[derive(Debug)]
pub(in crate::codegen::parser::html) struct StateMachine<'s> {
    // current dom.
    dom: HtmlDocument,
    // statck of nodes needs to be parsed.
    nodes: Vec<Node>,
    source: &'s str,
    state: State,
    current_attr_name: String,
    current_text: String,
}

impl<'s> StateMachine<'s> {
    pub(in crate::codegen::parser::html) fn new(source: &'s str) -> Self {
        Self {
            dom: HtmlDocument::default(),
            nodes: vec![],
            source: source,
            state: State::INIT,
            current_attr_name: "".into(),
            current_text: "".into(),
        }
    }

    pub(in crate::codegen::parser::html) fn process(&mut self) -> &HtmlDocument {
        let tokenizer = Tokenizer::new(self.source);
        let tokens = tokenizer.into_vec();
        let mut token_stream = TokenSlice::new(&tokens);
        while let Some(token) = token_stream.peek_token() {
            if token.kind() == tokenizer::Kind::EOF {
                self.transit_to(State::DONE);
                return &self.dom;
            }
            match self.state {
                State::INIT => self.process_with_init(&mut token_stream),
                State::TAGOPEN => self.process_with_tag_open(&mut token_stream),
                State::TAGNAME => self.process_with_tag_name(&mut token_stream),
                State::ATTRS => self.process_with_attributes(&mut token_stream),
                State::ATTRNAME => self.process_with_attr_name(&mut token_stream),
                State::ATTRVAL => self.process_with_attr_value(&mut token_stream),
                State::TEXT => self.process_with_text(&mut token_stream),
                State::TAGCLOSE => self.process_with_tag_close(&mut token_stream),
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
                    // here not sure whether it's comment or doctype.
                    // need to read ahead.
                    // consume the '!'
                    // todo: comment
                    token_stream.next_token();
                    self.transit_to(State::TAGNAME);
                }
                tokenizer::Kind::SLASH => {
                    token_stream.next_token();
                    self.transit_to(State::TAGCLOSE);
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
                    self.current_attr_name = attr_name.into();
                }
                tokenizer::Kind::EQUALS => {
                    token_stream.next_token();
                    self.transit_to(State::ATTRVAL);
                }
                tokenizer::Kind::GREATTHAN => {
                    token_stream.next_token();
                    self.transit_to(State::INIT);
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
                _ => {
                    // save text.
                    token_stream.next_token();
                    let part = &self.source[token.range()];
                    self.current_text.push_str(part);
                    self.state = State::TEXT;
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
                _ => {
                    token_stream.next_token();
                    self.transit_to(State::TAGCLOSE);
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
                while let Some(node) = self.nodes.pop() {
                    self.dom.push_node(node);
                }
            }
            State::INIT => {
                // a node closed
                if let Some(node) = self.nodes.last_mut() {
                    match node.kind() {
                        NodeKind::KELEMENT(_tage_name) => {
                            // check whether current is self-closed.
                            // remove from stack and push it to dom.
                            node.set_wellformed();
                            if node.is_closed() {
                                let current_node = self.nodes.pop().unwrap();
                                if let Some(parent_node) = self.nodes.last_mut() {
                                    parent_node.push_node(current_node);
                                } else {
                                    self.dom.push_node(current_node);
                                }
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
                        NodeKind::KTEXT(_) => {
                            // do nth.
                            let current_node = self.nodes.pop().unwrap();
                            if let Some(parent_node) = self.nodes.last_mut() {
                                parent_node.push_node(current_node);
                            } else {
                                self.dom.push_node(current_node);
                            }
                        }
                        NodeKind::KCOMMENT(_) => {
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
                if !self.current_text.is_empty() {
                    if let Some(node) = self.nodes.last_mut() {
                        match node.kind() {
                            NodeKind::KELEMENT(tage_name) => {
                                // todo: create text node
                                let text_content = if &tage_name.to_lowercase() == "pre" {
                                    &self.current_text
                                } else {
                                    self.current_text.trim()
                                };
                                if !text_content.is_empty() {
                                    let text_node = Node::new_text(text_content);
                                    node.push_node(text_node);
                                }
                            }
                            _ => {
                                let text_content = self.current_text.trim();
                                if !text_content.is_empty() {
                                    let text_node = Node::new_text(text_content);
                                    self.dom.push_node(text_node);
                                }
                            }
                        }
                    } else {
                        let text_content = self.current_text.trim();
                        if !text_content.is_empty() {
                            let text_node = Node::new_text(text_content);
                            self.dom.push_node(text_node);
                        }
                    }

                    self.current_text = "".into();
                }
            }
            _ => {}
        }

        self.state = state;
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
