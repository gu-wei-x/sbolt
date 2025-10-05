use crate::types::Writer;

impl Writer for String {
    fn into_string(self) -> String {
        self
    }

    fn write(&mut self, content: &str) {
        self.push_str(content);
    }
}
