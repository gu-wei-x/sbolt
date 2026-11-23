pub trait Writer {
    fn write(&mut self, content: &str);
    fn write_value(&mut self, value: &(impl super::WriteTo + ?Sized))
    where
        Self: Sized,
    {
        value.write_to(self);
    }
    /// Formats into this writer without creating an intermediate string.
    ///
    /// Panics if a value's `Display` implementation returns a formatting error.
    fn write_formatted(&mut self, args: std::fmt::Arguments<'_>) {
        struct Adapter<'a, W: ?Sized>(&'a mut W);

        impl<W: Writer + ?Sized> std::fmt::Write for Adapter<'_, W> {
            fn write_str(&mut self, content: &str) -> std::fmt::Result {
                self.0.write(content);
                Ok(())
            }
        }

        std::fmt::write(&mut Adapter(self), args).expect("formatting template value failed");
    }
    /// Writes explicitly trusted formatted output without HTML escaping.
    fn write_raw_formatted(&mut self, args: std::fmt::Arguments<'_>) {
        self.write_formatted(args);
    }
    fn writeln(&mut self, content: &str) {
        self.write(content);
        self.write("\n");
    }
    fn writefn(&mut self, content_fn: impl FnOnce() -> String) {
        self.write(&content_fn());
    }

    // convert to String by consuming self.
    fn into_string(self) -> String;
}

pub enum KWriter {
    KHtml(HtmlWriter),
    KJson(String),
    KText(String),
}

pub struct HtmlWriter {
    pub(crate) content: String,
}
