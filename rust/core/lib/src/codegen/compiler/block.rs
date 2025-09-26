use crate::codegen::parser::template::{Block, Kind};

impl<'a> Block<'a> {
    pub(crate) fn generate_code(&self, from: &Option<&Kind<'a>>, output: &mut String) {
        match *from {
            Some(from_kind) => {
                match (from_kind, self.span.kind()) {
                    (Kind::CODE(_), Kind::CODE(str)) => {
                        // TODO: put in a closure
                        output.push_str(str);
                    }
                    (Kind::CODE(_), Kind::CONTENT(str)) => {
                        // TODO: from code to content.put in a closure
                        // todo: inlined content sth like <View test="@code" .../>
                        // or content block.
                        output.push_str(&format!(r####"output.write(r#"{}"#);"####, str));
                    }
                    (Kind::CONTENT(_), Kind::INLINEDCODE(str)) => {
                        // code inside content.
                        // todo: check whether type is inlined.
                        // or block: block should be closure with output as parameter.
                        output.push_str(&format!(
                            r####"output.writefn(||({}).to_string());"####,
                            str
                        ));
                    }
                    (Kind::CONTENT(_), Kind::CONTENT(str)) => {
                        // from content to content.
                        output.push_str(&format!(r####"output.write(r#"{}"#);"####, str));
                    }
                    _ => {}
                }
            }
            None => {
                // from root.
                match self.span.kind() {
                    Kind::CODE(str) => {
                        if self.blocks.is_empty() {
                            //pure code block.
                            output.push_str(str);
                        } else {
                            // TODO: mixed code block. should be FnOnce with output as parameter.
                            for block in &self.blocks {
                                block.generate_code(&Some(&self.span.kind()), output);
                            }
                        }
                    }
                    Kind::CONTENT(str) => {
                        if self.blocks.is_empty() {
                            //pure content block.
                            output.push_str(&format!(r####"output.write(r#"{}"#);"####, str));
                        } else {
                            // TODO: mixed content block. should be FnOnce with output as parameter.
                            for block in &self.blocks {
                                block.generate_code(&Some(&self.span.kind()), output);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
