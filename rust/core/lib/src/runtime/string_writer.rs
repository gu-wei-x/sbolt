use crate::types::Writer;

impl Writer for String {
    fn into_string(self) -> String {
        self
    }

    fn write(&mut self, content: &str) {
        self.push_str(content);
    }

    fn write_formatted(&mut self, args: std::fmt::Arguments<'_>) {
        std::fmt::Write::write_fmt(self, args).expect("formatting template value failed");
    }
}

impl Writer for &mut String {
    fn into_string(self) -> String {
        std::mem::take(self)
    }

    fn write(&mut self, content: &str) {
        self.push_str(content);
    }

    fn write_formatted(&mut self, args: std::fmt::Arguments<'_>) {
        Writer::write_formatted(&mut **self, args);
    }
}
