use crate::types::error;
use std::collections::{HashMap, HashSet};

/// Measurements of generated static output, excluding dynamic expressions.
#[derive(Debug, Default, Clone)]
pub struct StaticOutputStats {
    input_bytes: usize,
    output_bytes: usize,
    input_writes: usize,
    output_writes: usize,
    unknown_context_writes: usize,
    literals: HashSet<String>,
}

impl StaticOutputStats {
    pub fn input_bytes(&self) -> usize {
        self.input_bytes
    }

    pub fn output_bytes(&self) -> usize {
        self.output_bytes
    }

    pub fn input_writes(&self) -> usize {
        self.input_writes
    }

    pub fn output_writes(&self) -> usize {
        self.output_writes
    }

    /// Writes preserved because their HTML insertion context is unknown.
    pub fn unknown_context_writes(&self) -> usize {
        self.unknown_context_writes
    }

    /// Distinct literal bytes; actual linker storage may differ.
    pub fn distinct_output_bytes(&self) -> usize {
        self.literals.iter().map(String::len).sum()
    }

    pub(in crate::codegen::compiler) fn record(
        &mut self,
        input: &str,
        input_writes: usize,
        output: &str,
        emitted: bool,
        unknown_context: bool,
    ) {
        self.input_bytes += input.len();
        self.input_writes += input_writes;
        if emitted {
            self.output_bytes += output.len();
            self.output_writes += 1;
            self.unknown_context_writes += usize::from(unknown_context);
            self.literals.insert(output.into());
        }
    }

    pub(in crate::codegen::compiler) fn merge(&mut self, other: Self) {
        self.input_bytes += other.input_bytes;
        self.output_bytes += other.output_bytes;
        self.input_writes += other.input_writes;
        self.output_writes += other.output_writes;
        self.unknown_context_writes += other.unknown_context_writes;
        self.literals.extend(other.literals);
    }
}

pub struct CompileResult {
    // errors which won't stop build.
    warnings: Vec<error::CompileError>,
    view_name_mapping: HashMap<String, String>,
    mods: Vec<String>,
    static_output_stats: StaticOutputStats,
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
            static_output_stats: StaticOutputStats::default(),
        }
    }

    pub fn add_warning(&mut self, error: error::CompileError) {
        self.warnings.push(error);
    }

    pub fn warnings(&self) -> &[error::CompileError] {
        &self.warnings
    }

    pub fn static_output_stats(&self) -> &StaticOutputStats {
        &self.static_output_stats
    }

    pub(in crate::codegen::compiler) fn add_static_output_stats(
        &mut self,
        stats: StaticOutputStats,
    ) {
        self.static_output_stats.merge(stats);
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
        self.static_output_stats.merge(other.static_output_stats);
        self.warnings.extend(other.warnings);
        self.view_name_mapping
            .extend(other.view_name_mapping.into_iter());
    }

    pub(crate) fn merge_into(self, other: &mut CompileResult) {
        other.merge_without_mods(self);
    }
}
