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

// Wrapper type for Option<&T> and Option<T> to implement Display trait.
pub struct DisplayOption<T: std::fmt::Display>(pub Option<T>);
impl<T: std::fmt::Display> From<T> for DisplayOption<T> {
    fn from(value: T) -> Self {
        DisplayOption(Some(value))
    }
}

impl<T: std::fmt::Display> std::fmt::Display for DisplayOption<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "{}", value),
            None => write!(f, ""),
        }
    }
}

pub struct DisplayOptionRef<'a, T: std::fmt::Display>(pub Option<&'a T>);
impl<'a, T: std::fmt::Display> From<&'a T> for DisplayOptionRef<'a, T> {
    fn from(value: &'a T) -> Self {
        DisplayOptionRef(Some(value))
    }
}
impl<'a, T: std::fmt::Display> std::fmt::Display for DisplayOptionRef<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{}", value),
            None => write!(f, ""),
        }
    }
}
