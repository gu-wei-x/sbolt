use crate::codegen::{
    CompilerOptions,
    parser::{Token, optimizer::Optimizer, tokenizer::Kind},
};

pub struct HtmlParseContext {
    pre: Option<Token>,
    tag: Option<Token>,
    is_closed: bool,
    is_close_tag: bool,
    is_self_close_tag: bool,
    is_start_tag: bool,
    is_quoted: bool,
}

impl Default for HtmlParseContext {
    fn default() -> Self {
        Self {
            pre: None,
            tag: None,
            is_closed: false,
            is_close_tag: false,
            is_self_close_tag: false,
            is_start_tag: false,
            is_quoted: false,
        }
    }
}

impl HtmlParseContext {
    fn close(&mut self) {
        self.is_closed = true;
    }

    fn set_pre(&mut self, pre: Token) {
        self.pre = Some(pre);
    }

    fn set_tag(&mut self, tag: Token) {
        self.tag = Some(tag);
    }

    fn start(&mut self) {
        self.is_start_tag = true;
    }

    fn set_close_tag(&mut self) {
        self.is_close_tag = true;
    }
}

pub(in crate::codegen::parser::optimizer) struct HtmlOptimizer<'a> {
    compiler_options: &'a CompilerOptions,
    contexts: Vec<HtmlParseContext>,
}

impl<'a> HtmlOptimizer<'a> {
    pub(in crate::codegen::parser::optimizer) fn new(
        compiler_options: &'a CompilerOptions,
    ) -> Self {
        Self {
            compiler_options: compiler_options,
            contexts: vec![],
        }
    }
}

impl<'a> Optimizer for HtmlOptimizer<'a> {
    fn accept<'s>(&mut self, source: &'s str, token: &Token) -> bool {
        // EOF is special.
        if token.kind() == Kind::EOF {
            return false;
        }

        if !self.compiler_options.need_optimization() {
            return true;
        }

        // todo: cache some tokens for refer to decide whether to accep a token.
        // consume to return all tokens.
        //<div><div>test</div></div>
        //<div class="", t=""></div>
        match token.kind() {
            Kind::LESSTHAN => {
                // open tag: create new context, don't know whether the tag is start or close.
                if self.contexts.is_empty() || self.contexts.last().unwrap().is_closed {
                    let mut context = HtmlParseContext::default();
                    context.set_pre(*token);
                    self.contexts.push(context);
                }

                // accept.
                true
            }
            Kind::GREATTHAN => {
                // close tag
                if let Some(context) = self.contexts.last_mut() {
                    // pop as to finish current context.
                    if context.is_close_tag {
                        // close tag.
                        self.contexts.pop();
                        // start tag.
                        // todo: pop if tag name matches
                        self.contexts.pop();
                    } else if context.is_self_close_tag {
                        // start tag.
                        self.contexts.pop();
                    } else if let Some(token) = context.tag {
                        // close the start tag.
                        context.close();

                        // handle tags don't need close it self like link, meta, br
                        let tag_name = source[token.range()].to_lowercase();
                        let tags = [
                            String::from("br"),
                            String::from("doctype"),
                            String::from("link"),
                            String::from("meta"),
                        ];
                        if tags.contains(&tag_name) {
                            self.contexts.pop();
                        }
                    }
                }

                // accept.
                true
            }
            Kind::SLASH => {
                // </div>: close tag
                // <div a="a1" ... />: self-close tag
                // <br/>: self-close tag
                if let Some(context) = self.contexts.last_mut() {
                    if let Some(pre) = context.pre {
                        match pre.kind() {
                            Kind::LESSTHAN => {
                                // like: </div>, current is close tag.
                                context.set_close_tag();
                            }
                            Kind::EXPRESSION => {
                                // self colse tag.
                                context.is_self_close_tag = true;
                            }
                            _ => {}
                        }
                    }
                    context.set_pre(*token);
                }
                true
            }
            Kind::EXPRESSION => {
                let exp = &source[token.range()];
                let is_quoted_started = exp.starts_with("\"") || exp.starts_with("\'");
                let is_quoted_ended = exp.ends_with("\"") || exp.ends_with("\'");
                if let Some(context) = self.contexts.last_mut() {
                    if let Some(pre) = context.pre
                        && pre.kind() == Kind::LESSTHAN
                        && !(is_quoted_ended || is_quoted_started)
                    {
                        // tag nme.
                        context.set_tag(*token);
                        context.start();
                    } else if context.is_start_tag {
                        context.pre = Some(*token);
                        if is_quoted_started {
                            context.is_quoted = is_quoted_started;
                        }
                        if context.is_quoted {
                            context.is_quoted = !is_quoted_ended;
                        }
                    }

                    context.set_pre(*token);
                }

                // accept.
                true
            }
            Kind::WHITESPACE => {
                if let Some(context) = self.contexts.last_mut() {
                    // inside quoted string
                    if context.is_quoted {
                        context.set_pre(*token);
                        true
                    } else if context.is_closed {
                        false
                    } else {
                        if let Some(tag_token) = context.tag {
                            let token_name = &source[tag_token.range()];
                            if token_name.to_lowercase() == "pre" {
                                context.set_pre(*token);
                                return true;
                            } else if let Some(pre) = context.pre
                                && pre.kind() != Kind::WHITESPACE
                            {
                                context.set_pre(*token);
                                return true;
                            }
                        }
                        false
                    }
                } else {
                    false
                }
            }
            Kind::NEWLINE => false,
            _ => true,
        }
    }
}
