use crate::types::HtmlWriter;
use crate::types::Writer;

impl Writer for HtmlWriter {
    fn into_string(self) -> String {
        self.content
    }

    // TODO: do optimization here for html stream.
    fn write(&mut self, content: &str) {
        self.content.push_str(content);
    }
}

impl HtmlWriter {
    pub fn new() -> Self {
        HtmlWriter {
            content: String::new(),
        }
    }
}
