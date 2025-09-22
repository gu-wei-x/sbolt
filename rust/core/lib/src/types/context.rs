use crate::types::{DataStore, Writer};
use std::fmt::{self, Display, Formatter};

pub trait Context: Writer {
    fn set_data<T, F>(&mut self, key: &str, f: F)
    where
        F: FnOnce() -> T,
        T: Send + Sync + 'static;

    fn get_data<T>(&self, key: &str) -> &T
    where
        T: Send + Sync + 'static;
}

pub struct DefaultViewContext {
    pub output: String,
    pub(crate) state: DataStore<String>,
}

impl DefaultViewContext {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            state: DataStore::<String>::new(),
        }
    }
}

impl Context for DefaultViewContext {
    fn set_data<T, F>(&mut self, key: &str, f: F)
    where
        F: FnOnce() -> T,
        T: Send + Sync + 'static,
    {
        self.state.set(key, f());
    }

    fn get_data<T>(&self, key: &str) -> &T
    where
        T: Send + Sync + 'static,
    {
        self.state.get(key).unwrap()
    }
}

impl Writer for DefaultViewContext {
    fn write(&mut self, content: &str) {
        self.output.push_str(content);
    }

    fn writeln(&mut self, content: &str) {
        self.write(content);
        self.write("\n");
    }
}

impl Display for DefaultViewContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.output)
    }
}
