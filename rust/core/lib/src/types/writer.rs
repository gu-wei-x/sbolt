pub trait Writer {
    fn write(&mut self, content: &str);
    fn write_line(&mut self, content: &str);
}

impl Writer for String {
    fn write(&mut self, content: &str) {
        self.push_str(content);
    }

    fn write_line(&mut self, content: &str) {
        self.write(content);
        self.write("\n");
    }
}
