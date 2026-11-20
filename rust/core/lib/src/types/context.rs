use crate::types::DataStore;
use std::collections::HashMap;

pub trait Context {
    /// Estimated total output bytes, including dynamic content and loops.
    fn capacity_hint(&self) -> usize {
        0
    }

    /// Reborrow an owned or already-borrowed context for nested rendering.
    fn as_mut_context(&mut self) -> &mut Self {
        self
    }

    fn set_data<T, F>(&mut self, key: &str, f: F)
    where
        F: FnOnce() -> T,
        T: Send + Sync + 'static;

    fn get_data<T>(&self, key: &str) -> Option<&T>
    where
        T: Send + Sync + 'static;

    fn add_section(&mut self, name: &str, content: String) -> &mut Self;
    fn get_section(&self, name: &str) -> Option<&Vec<String>>;
    fn get_section_mut(&mut self, name: &str) -> Option<&mut Vec<String>>;
    fn get_default_section(&self) -> Option<&String>;
    fn set_default_section(&mut self, content: String) -> &mut Self;
}

pub struct DefaultViewContext {
    pub(crate) state: DataStore<Box<dyn Send + Sync>>,
    sections: HashMap<String, Vec<String>>,
    capacity_hint: usize,
}

impl DefaultViewContext {
    pub fn new() -> Self {
        Self {
            state: DataStore::<Box<dyn Send + Sync>>::new(),
            sections: HashMap::<String, Vec<String>>::new(),
            capacity_hint: 0,
        }
    }

    /// Sets the output estimate for renders and layouts sharing this context.
    /// Section buffers keep their own compile-time estimates.
    pub fn set_capacity_hint(&mut self, capacity_hint: usize) -> &mut Self {
        self.capacity_hint = capacity_hint;
        self
    }
}

impl Context for DefaultViewContext {
    fn capacity_hint(&self) -> usize {
        self.capacity_hint
    }

    fn set_data<T, F>(&mut self, key: &str, f: F)
    where
        F: FnOnce() -> T,
        T: Send + Sync + 'static,
    {
        self.state.set(key, f());
    }

    fn get_data<T>(&self, key: &str) -> Option<&T>
    where
        T: Send + Sync + 'static,
    {
        self.state.get(key)
    }

    fn add_section(&mut self, name: &str, content: String) -> &mut Self {
        self.sections
            .entry(name.to_owned())
            .or_default()
            .push(content);
        self
    }

    fn get_section(&self, name: &str) -> Option<&Vec<String>> {
        self.sections.get(name)
    }

    fn get_section_mut(&mut self, name: &str) -> Option<&mut Vec<String>> {
        self.sections.get_mut(name)
    }

    fn get_default_section(&self) -> Option<&String> {
        match self.get_section("default") {
            Some(defaults) => defaults.last(),
            None => None,
        }
    }

    fn set_default_section(&mut self, content: String) -> &mut Self {
        match self.get_section_mut("default") {
            Some(defaults) => {
                defaults.clear();
                defaults.push(content);
            }
            None => {
                self.add_section("default", content);
            }
        }
        self
    }
}
