use crate::codegen::parser::template::{Block, Kind};

impl<'a> Block<'a> {
    pub(crate) fn generate_code(&self, from: Option<Kind>, output: &mut String) {
        match from {
            Some(from_kind) => {
                match (from_kind, self.kind()) {
                    (Kind::CODE, Kind::CODE) => {
                        // TODO: put in a closure
                        output.push_str(self.content());
                    }
                    (Kind::CODE, Kind::CONTENT) => {
                        // TODO: from code to content.put in a closure
                        // todo: inlined content sth like <View test="@code" .../>
                        // or content block.
                        println!("???????????????????exepected: {}", self.content());
                        output
                            .push_str(&format!(r####"output.write(r#"{}"#);"####, self.content()));
                    }
                    (Kind::CONTENT, Kind::INLINEDCODE) => {
                        // code inside content.
                        // todo: check whether type is inlined.
                        // or block: block should be closure with output as parameter.
                        output.push_str(&format!(
                            r####"output.writefn(||({}).to_string());"####,
                            self.content()
                        ));
                    }
                    (Kind::CONTENT, Kind::CONTENT) => {
                        // from content to content.
                        output
                            .push_str(&format!(r####"output.write(r#"{}"#);"####, self.content()));
                    }
                    (_, _) => {
                        println!(
                            "+++++++++++++++++++++++++++++exepected:{from_kind:?}, {:?}",
                            self.content()
                        )
                    }
                }
            }
            None => {
                // from root.
                match self.kind() {
                    Kind::CODE => {
                        if !self.has_blocks() {
                            //pure code block.
                            output.push_str(self.content());
                        } else {
                            // TODO: mixed code block. should be FnOnce with output as parameter.
                            for block in self.blocks() {
                                block.generate_code(Some(Kind::CODE), output);
                            }
                        }
                    }
                    Kind::CONTENT => {
                        if !self.has_blocks() {
                            //pure content block.
                            output.push_str(&format!(
                                r####"output.write(r#"{}"#);"####,
                                self.content()
                            ));
                        } else {
                            // TODO: mixed content block. should be FnOnce with output as parameter.
                            for block in self.blocks() {
                                block.generate_code(Some(Kind::CONTENT), output);
                            }
                        }
                    }
                    Kind::INLINEDCODE => {
                        output.push_str(&format!(
                            r####"output.writefn(||({}).to_string());"####,
                            self.content()
                        ));
                    }
                    _ => {}
                }
            }
        }
    }
}
