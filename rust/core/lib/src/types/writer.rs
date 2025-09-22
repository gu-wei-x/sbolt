pub trait Writer {
    fn write(&mut self, content: &str);
    fn writeln(&mut self, content: &str);
    fn writefn(&mut self, content_fn: impl FnOnce() -> String) {
        self.write(&content_fn());
    }
}

impl Writer for String {
    fn write(&mut self, content: &str) {
        self.push_str(content);
    }

    fn writeln(&mut self, content: &str) {
        self.write(content);
        self.write("\n");
    }
}
