pub trait Writer {
    fn write(&mut self, content: &str);
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

pub struct HtmlWriter {
    pub(crate) content: String,
}

// Wrapper type for Option<&T> and Option<T> to implement Display trait.
pub struct DisplayOption<T: std::fmt::Display>(Option<T>);
impl<T: std::fmt::Display> From<T> for DisplayOption<T> {
    fn from(value: T) -> Self {
        DisplayOption(Some(value))
    }
}

impl<T: std::fmt::Display> std::fmt::Display for DisplayOption<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{}", value),
            None => write!(f, ""),
        }
    }
}
