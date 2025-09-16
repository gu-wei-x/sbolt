use crate::codegen::consts;

pub struct CompilerOptions {
    pub(crate) optimize: bool,
    pub(crate) debug: bool,
    pub(crate) source_dirs: Vec<String>,
    pub(crate) extensions: Vec<String>,
    pub(crate) out_dir: Option<String>,
    pub(crate) mod_name: String,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        let mut options = CompilerOptions {
            optimize: false,
            debug: false,
            source_dirs: Vec::new(),
            extensions: Vec::new(),
            out_dir: None,
            mod_name: String::from("generated_views"),
        };
        options
            .extensions
            .push(consts::DEFAULT_TEMPLATE_FILE_EXTENSION.into());
        options
    }
}

impl CompilerOptions {
    pub fn with_optimize(mut self, optimize: bool) -> Self {
        self.optimize = optimize;
        self
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn with_source_dir(mut self, source_dir: &str) -> Self {
        self.source_dirs.push(source_dir.into());
        self
    }

    pub fn with_extension(mut self, extension: &str) -> Self {
        self.extensions.push(extension.into());
        self
    }

    pub fn with_out_dir(mut self, out_dir: &str) -> Self {
        self.out_dir = Some(out_dir.into());
        self
    }

    pub fn with_mod_name(mut self, mod_name: &str) -> Self {
        self.mod_name = mod_name.into();
        self
    }
}
