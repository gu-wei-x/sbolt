use crate::{
    codegen::parser::{
        Token,
        html::{
            doc::HtmlDocument,
            node::{Node, NodeKind},
        },
        tokenizer::{self, Kind, TokenStream, Tokenizer},
    },
    types::result,
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
    COMMENT,
    DONE,
}

#[derive(Debug)]
pub(in crate::codegen::parser::html) struct StateMachine<'s> {
    // statck of nodes needs to be parsed.
    nodes: Vec<Node>,
    pending_name: String,
    source: &'s str,
    state: State,
}

impl<'s> StateMachine<'s> {
    pub(in crate::codegen::parser::html) fn new(source: &'s str) -> Self {
        Self {
            nodes: vec![],
            pending_name: "".into(),
            source: source,
            state: State::INIT,
        }
    }

    pub(in crate::codegen::parser::html) fn process(&mut self) -> result::Result<HtmlDocument> {
        let tokenizer = Tokenizer::new(self.source);
        let tokens = tokenizer.into_vec();
        let mut token_stream = TokenSlice::new(&tokens);
        let mut dom = HtmlDocument::default();
        while let Some(token) = token_stream.peek_token() {
            if token.kind() == tokenizer::Kind::EOF {
                self.transit_to(State::DONE, &mut dom);
                break;
            }
            match self.state {
                State::INIT => self.process_with_init(&mut token_stream, &mut dom),
                State::TAGOPEN => self.process_with_tag_open(&mut token_stream, &mut dom),
                State::TAGNAME => self.process_with_tag_name(&mut token_stream, &mut dom)?,
                State::ATTRS => self.process_with_attributes(&mut token_stream, &mut dom),
                State::ATTRNAME => self.process_with_attr_name(&mut token_stream, &mut dom),
                State::ATTRVAL => self.process_with_attr_value(&mut token_stream, &mut dom),
                State::TEXT => self.process_with_text(&mut token_stream, &mut dom),
                State::TAGCLOSE => self.process_with_tag_close(&mut token_stream, &mut dom),
                State::INSTSTART => {
                    self.process_with_instruction_start(&mut token_stream, &mut dom)?
                }
                State::COMMENT => self.process_with_comment(&mut token_stream, &mut dom),
                State::DONE => {
                    self.transit_to(State::DONE, &mut dom);
                    break;
                }
            }
        }
        Ok(dom)
    }
}

impl<'s> StateMachine<'s> {
    fn process_with_init(&mut self, token_stream: &mut TokenStream, dom: &mut HtmlDocument) {
        // we know there is valid token;
        let token = token_stream.peek_token().unwrap();
        match token.kind() {
            tokenizer::Kind::LESSTHAN => {
                // consmue '<'
                token_stream.next_token();
                self.transit_to(State::TAGOPEN, dom);
            }
            _ => {
                self.transit_to(State::TEXT, dom);
            }
        }
    }

    fn process_with_tag_open(&mut self, token_stream: &mut TokenStream, dom: &mut HtmlDocument) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::EXCLAMATION => {
                    // doctype or comment
                    token_stream.next_token();
                    self.transit_to(State::INSTSTART, dom);
                }
                tokenizer::Kind::SLASH => {
                    token_stream.next_token();
                    self.transit_to(State::TAGCLOSE, dom);
                }
                _ if is_valid_name_token(token) => {
                    self.transit_to(State::TAGNAME, dom);
                }
                _ => {
                    token_stream.next_token();
                    self.transit_to(State::TAGOPEN, dom);
                }
            }
        } else {
            self.transit_to(State::DONE, dom);
        }
    }

    fn process_with_tag_name(
        &mut self,
        token_stream: &mut TokenStream,
        dom: &mut HtmlDocument,
    ) -> result::Result<()> {
        // here we know there is valid token.
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::GREATTHAN => {
                    let node = Node::new_element(&self.pending_name);
                    self.nodes.push(node);
                    self.pending_name.clear();

                    token_stream.next_token();
                    self.transit_to(State::INIT, dom);
                }
                tokenizer::Kind::WHITESPACE | tokenizer::Kind::NEWLINE => {
                    let node = Node::new_element(&self.pending_name);
                    self.nodes.push(node);
                    self.pending_name.clear();

                    token_stream.next_token();
                    self.transit_to(State::ATTRS, dom);
                }
                _ if is_valid_name_token(token) => {
                    let name_part = &self.source[token.range()];
                    self.pending_name.push_str(&name_part);
                    token_stream.next_token();
                }
                _ => {
                    // ignore.
                    return Err(crate::types::error::CompileError::from_parser(
                        self.source,
                        Some(*token),
                        "Illegal char",
                    ));
                }
            }
        } else {
            self.transit_to(State::DONE, dom);
        }

        Ok(())
    }

    fn process_with_attributes(&mut self, token_stream: &mut TokenStream, dom: &mut HtmlDocument) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::GREATTHAN => {
                    token_stream.next_token();
                    self.transit_to(State::INIT, dom);
                }
                _ if is_valid_name_token(token) => self.transit_to(State::ATTRNAME, dom),
                _ => {
                    token_stream.next_token();
                }
            }
        } else {
            self.transit_to(State::DONE, dom);
        }
    }

    fn process_with_attr_name(&mut self, token_stream: &mut TokenStream, dom: &mut HtmlDocument) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::EQUALS => {
                    token_stream.next_token();
                    self.transit_to(State::ATTRVAL, dom);
                }
                tokenizer::Kind::GREATTHAN => {
                    let node = self.nodes.last_mut().unwrap();
                    node.push_attr(&self.pending_name, "");
                    self.pending_name.clear();
                    token_stream.next_token();
                    self.transit_to(State::INIT, dom);
                }
                tokenizer::Kind::WHITESPACE | tokenizer::Kind::NEWLINE => {
                    let node = self.nodes.last_mut().unwrap();
                    node.push_attr(&self.pending_name, "");
                    self.pending_name.clear();

                    // keep state for next name.
                    token_stream.next_token();
                }
                // doctype attr_name could be quotated.
                tokenizer::Kind::DQMARK => {
                    let attr_name = get_possible_quotated_string_from_stream(
                        self.source,
                        token_stream,
                        tokenizer::Kind::DQMARK,
                    );
                    self.nodes
                        .last_mut()
                        .unwrap()
                        .push_attr(&format!("\"{}\"", attr_name), "");
                    self.pending_name.clear();
                    self.transit_to(State::ATTRNAME, dom);
                }
                tokenizer::Kind::SQMAERK => {
                    let attr_name = get_possible_quotated_string_from_stream(
                        self.source,
                        token_stream,
                        tokenizer::Kind::SQMAERK,
                    );
                    self.nodes
                        .last_mut()
                        .unwrap()
                        .push_attr(&format!("'{}'", attr_name), "");
                    self.pending_name.clear();
                    self.transit_to(State::ATTRNAME, dom);
                }
                _ if is_valid_name_token(token) => {
                    let attr_name = &self.source[token.range()];
                    self.pending_name.push_str(attr_name);
                    token_stream.next_token();
                }
                _ => {
                    token_stream.next_token();
                }
            }
        } else {
            self.transit_to(State::DONE, dom);
        }
    }

    fn process_with_attr_value(&mut self, token_stream: &mut TokenStream, dom: &mut HtmlDocument) {
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
                        .push_attr(&self.pending_name, &attr_value);
                    self.pending_name.clear();
                    self.transit_to(State::ATTRNAME, dom);
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
                        .push_attr(&self.pending_name, &attr_value);
                    self.pending_name.clear();
                    self.transit_to(State::ATTRNAME, dom);
                }
                tokenizer::Kind::GREATTHAN => {
                    token_stream.next_token();
                    self.transit_to(State::INIT, dom);
                }
                _ => {
                    token_stream.next_token();
                    self.transit_to(State::ATTRNAME, dom);
                }
            }
        } else {
            self.transit_to(State::DONE, dom);
        }
    }

    fn process_with_text(&mut self, token_stream: &mut TokenStream, dom: &mut HtmlDocument) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::LESSTHAN => {
                    // consmue '<'
                    token_stream.next_token();
                    self.transit_to(State::TAGOPEN, dom);
                }
                tokenizer::Kind::EOF => {
                    self.transit_to(State::DONE, dom);
                }
                _ => {
                    // save text.
                    token_stream.next_token();
                    let part = &self.source[token.range()];
                    self.nodes.last_mut().unwrap().push_text(&part);
                }
            }
        } else {
            self.transit_to(State::DONE, dom);
        }
    }

    fn process_with_tag_close(&mut self, token_stream: &mut TokenStream, dom: &mut HtmlDocument) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::GREATTHAN => {
                    // closed
                    token_stream.next_token();
                    self.transit_to(State::INIT, dom);
                }
                _ if is_valid_name_token(token) => {
                    let tag_name = &self.source[token.range()];
                    let node = Node::new_close_element(tag_name);
                    self.nodes.push(node);

                    // todo: tag name but tag is close tag.

                    token_stream.next_token();
                }
                _ => {
                    token_stream.next_token();
                }
            }
        } else {
            self.transit_to(State::DONE, dom);
        }
    }

    fn process_with_instruction_start(
        &mut self,
        token_stream: &mut TokenStream,
        dom: &mut HtmlDocument,
    ) -> result::Result<()> {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::HYPHEN => {
                    // comment.
                    let node = Node::new_comment();
                    self.nodes.push(node);
                    self.transit_to(State::COMMENT, dom);
                }
                tokenizer::Kind::GREATTHAN => {
                    // closed
                    token_stream.next_token();
                    self.transit_to(State::INIT, dom);
                }
                _ if is_valid_name_token(token) => {
                    self.transit_to(State::TAGNAME, dom);
                }
                _ => {
                    return Err(crate::types::error::CompileError::from_parser(
                        self.source,
                        Some(*token),
                        "Illegal char",
                    ));
                }
            }
        } else {
            self.transit_to(State::DONE, dom);
        }

        Ok(())
    }

    fn process_with_comment(&mut self, token_stream: &mut TokenStream, dom: &mut HtmlDocument) {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::GREATTHAN => {
                    // closed
                    token_stream.next_token();
                    self.transit_to(State::INIT, dom);
                }
                _ => {
                    let parts = &self.source[token.range()];
                    self.nodes.last_mut().unwrap().push_text(parts);
                    token_stream.next_token();
                }
            }
        } else {
            self.transit_to(State::DONE, dom);
        }
    }

    fn transit_to(&mut self, state: State, dom: &mut HtmlDocument) {
        match state {
            State::DONE => {
                // unwind last one.
                let current_node = self.nodes.pop();
                if let Some(node) = current_node {
                    self.push_to_dom_or_unwind(node, dom);
                }

                // push all remaining node.
                self.nodes.reverse();
                while let Some(node) = self.nodes.pop() {
                    dom.push_node(node);
                }
            }
            State::INIT => {
                // saw '>' and '>' was consumed by caller, need to check the stack.
                if let Some(node) = self.nodes.last_mut() {
                    match node.kind() {
                        NodeKind::KELEMENT(tag_name) => {
                            // check whether current is self-closed.
                            // remove from stack and push it to dom.
                            node.set_wellformed();
                            let tag_name = &tag_name.to_lowercase();
                            if node.is_closed() {
                                let current_node = self.nodes.pop().unwrap();
                                self.push_to_dom_or_unwind(current_node, dom);
                            } else if is_void_tag(tag_name) {
                                // a following close tag for void is illegal, leave it to browser.
                                let mut current_node = self.nodes.pop().unwrap();
                                current_node.close();
                                self.push_to_dom_or_unwind(current_node, dom);
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
                                        self.push_to_dom_or_unwind(current_node, dom);
                                    }
                                    _ => {
                                        // doen't match: <div>test</a> ignore.
                                    }
                                }
                            } else {
                                // </a>< -- pre might be generated at runtime.
                                dom.push_node(close_node);
                            }
                        }
                        NodeKind::KCOMMENT => {
                            // comment is a special node <!----> end with '>'.
                            // doesn't matter whether close or wellformed.
                            node.set_wellformed();
                            node.close();
                            let current_node = self.nodes.pop().unwrap();
                            self.push_to_dom_or_unwind(current_node, dom);
                        }
                        NodeKind::KTEXT => {
                            // impossible as '>' would be treated as text.
                            // do nth here to have full arms.
                        }
                    }
                }
            }
            State::TAGOPEN => {
                // saw '<' and '<' was consumed by caller, need to check the stack.
                // for text: need to add it to dom
                // scenario: node<,
                let current_node = self.nodes.last_mut();
                if let Some(node) = current_node
                    && node.kind() == NodeKind::KTEXT
                {
                    // pop from stack and add to dom tree.
                    let current_node = self.nodes.pop().unwrap();
                    self.push_to_dom_or_unwind(current_node, dom);
                }
            }
            State::TEXT => {
                let node = Node::new_text();
                self.nodes.push(node);
            }
            _ => {
                // do nth.
            }
        }

        self.state = state;
    }
    fn push_to_dom_or_unwind(&mut self, node: Node, dom: &mut HtmlDocument) {
        if let Some(parent_node) = self.nodes.last_mut() {
            parent_node.push_node(node);
        } else {
            dom.push_node(node);
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

static VOID_TAGS: [&'static str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];
fn is_void_tag(tag_name: &str) -> bool {
    VOID_TAGS.contains(&tag_name)
}

fn is_valid_name_token(token: &Token) -> bool {
    // here: don't valid whether it's a html name, will put all tokens in name
    // and let browser to decide it.
    token.kind() != tokenizer::Kind::WHITESPACE && token.kind() != tokenizer::Kind::NEWLINE
}
