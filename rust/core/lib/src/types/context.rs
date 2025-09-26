use crate::types::DataStore;

pub trait Context {
    fn set_data<T, F>(&mut self, key: &str, f: F)
    where
        F: FnOnce() -> T,
        T: Send + Sync + 'static;

    fn get_data<T>(&self, key: &str) -> &T
    where
        T: Send + Sync + 'static;
}

pub struct DefaultViewContext {
    pub(crate) state: DataStore<String>,
}

impl DefaultViewContext {
    pub fn new() -> Self {
        Self {
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
