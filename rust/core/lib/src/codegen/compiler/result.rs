use std::collections::HashMap;

pub struct CompileResult {
    errors: Vec<String>,
    view_name_mapping: HashMap<String, String>,
    mods: Vec<String>,
}

impl Default for CompileResult {
    fn default() -> Self {
        Self::new()
    }
}

impl CompileResult {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            view_name_mapping: HashMap::new(),
            mods: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}

impl CompileResult {
    pub(crate) fn view_name_mapping(&self) -> &HashMap<String, String> {
        &self.view_name_mapping
    }

    pub(crate) fn add_view_mapping(&mut self, view_name: String, file_path: String) {
        self.view_name_mapping.insert(view_name, file_path);
    }

    pub(crate) fn add_mod(&mut self, mod_name: &str) {
        let name = mod_name.to_string();
        if !self.mods.contains(&name) {
            self.mods.push(name);
        }
    }

    pub(crate) fn mods(&self) -> &[String] {
        &self.mods
    }
}
