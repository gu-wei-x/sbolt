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

    fn write_formatted(&mut self, args: std::fmt::Arguments<'_>) {
        std::fmt::Write::write_fmt(&mut self.content, args)
            .expect("formatting template value failed");
    }
}

impl HtmlWriter {
    /// Creates a writer with an optional output-capacity estimate in bytes.
    pub fn new(capacity_hint: Option<usize>) -> Self {
        HtmlWriter {
            content: String::with_capacity(capacity_hint.unwrap_or_default()),
        }
    }
}
