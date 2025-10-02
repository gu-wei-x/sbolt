use crate::types::error;
use std::collections::HashMap;

pub struct CompileResult {
    // errors which won't stop build.
    warnings: Vec<error::CompileError>,
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
            warnings: Vec::new(),
            view_name_mapping: HashMap::new(),
            mods: Vec::new(),
        }
    }

    pub fn add_warning(&mut self, error: error::CompileError) {
        self.warnings.push(error);
    }

    pub fn is_success(&self) -> bool {
        self.warnings.is_empty()
    }

    pub fn warnings(&self) -> &[error::CompileError] {
        &self.warnings
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

    pub(crate) fn merge_without_mods(&mut self, other: CompileResult) {
        self.warnings.extend(other.warnings);
        self.view_name_mapping
            .extend(other.view_name_mapping.into_iter());
    }

    pub(crate) fn merge_into(self, other: &mut CompileResult) {
        other.merge_without_mods(self);
    }
}
