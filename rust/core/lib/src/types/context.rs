use crate::types::DataStore;
use std::collections::HashMap;

pub trait Context {
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
    pub(crate) state: DataStore<String>,
    sections: HashMap<String, Vec<String>>,
}

impl DefaultViewContext {
    pub fn new() -> Self {
        Self {
            state: DataStore::<String>::new(),
            sections: HashMap::<String, Vec<String>>::new(),
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

    fn get_data<T>(&self, key: &str) -> Option<&T>
    where
        T: Send + Sync + 'static,
    {
        self.state.get(key)
    }

    fn add_section(&mut self, name: &str, content: String) -> &mut Self {
        self.sections
            .entry(name.to_owned())
            .or_insert(vec![content]);
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
