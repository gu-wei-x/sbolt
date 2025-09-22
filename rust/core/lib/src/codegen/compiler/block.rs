use crate::codegen::parser::template::{Block, Kind};

impl<'a> Block<'a> {
    pub(crate) fn get_content(&self) -> &'a str {
        match self.span.kind() {
            Kind::CODE(str) => str,
            Kind::CONTENT(str) => str,
            Kind::DOC(str) => str,
            Kind::MIXED => {
                // todo:
                ""
            }
        }
    }

    pub(crate) fn generate_code(&self, output: &mut String) {
        if self.blocks.len() == 1 && matches!(self.span.kind(), Kind::DOC(_)) {
            output.push_str(&format!(
                r####"context.write(r#"{}"#);"####,
                self.get_content()
            ));
        } else {
            let from = self.span.kind();
            for block in &self.blocks {
                block.render_content(&from, output);
            }
        }
    }

    pub(crate) fn render_content(&self, from: &Kind<'a>, output: &mut String) {
        match self.span.kind() {
            Kind::CODE(str) => {
                match from {
                    Kind::CODE(_) | Kind::CONTENT(_) => {
                        //unexpected.
                    }
                    Kind::DOC(_) => output.push_str(str),
                    Kind::MIXED => {
                        output.push_str(&format!(
                            r####"context.writefn(||({}).to_string());"####,
                            str
                        ));
                    }
                }
            }
            Kind::CONTENT(str) => match from {
                Kind::CODE(_) | Kind::CONTENT(_) => {}
                Kind::DOC(_) => {
                    output.push_str(&format!(r####"context.write(r#"{}"#);"####, str));
                }
                Kind::MIXED => {
                    output.push_str(&format!(r####"context.write(r#"{}"#);"####, str));
                }
            },
            Kind::DOC(_str) => {
                match from {
                    Kind::CODE(_) | Kind::DOC(_) | Kind::CONTENT(_) | Kind::MIXED => { /*unexpected */
                    }
                }
            }
            Kind::MIXED => {
                let mut result = String::new();
                for block in &self.blocks {
                    block.render_content(&self.span.kind(), &mut result);
                }
                output.push_str(&result);
            }
        }
    }
}
