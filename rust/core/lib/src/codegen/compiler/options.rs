use crate::{codegen::consts, types::template};
use std::collections::HashMap;

pub struct CompilerOptions {
    pub(crate) debug: bool,
    pub(crate) extensions: HashMap<String, template::Kind>,
    pub(crate) mod_name: String,
    pub(crate) optimize: bool,
    pub(crate) out_dir: Option<String>,
    pub(crate) source_dirs: Vec<String>,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        let options = CompilerOptions {
            debug: false,
            extensions: HashMap::<String, template::Kind>::new(),
            mod_name: String::from(consts::TEMP_GENERATED_DIR),
            optimize: false,
            out_dir: None,
            source_dirs: Vec::new(),
        };

        // add default extensions and their kinds
        let options = options
            .with_extension(
                consts::DEFAULT_HTML_TEMPLATE_FILE_EXTENSION,
                template::Kind::KHTML,
            )
            .with_extension(
                consts::DEFAULT_JSON_TEMPLATE_FILE_EXTENSION,
                template::Kind::KJSON,
            )
            .with_extension(
                consts::DEFAULT_TEXT_TEMPLATE_FILE_EXTENSION,
                template::Kind::KTEXT,
            );
        options
    }
}

impl CompilerOptions {
    pub fn extensions(&self) -> &HashMap<String, template::Kind> {
        &self.extensions
    }

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

    pub fn with_extension(mut self, extension: &str, kind: template::Kind) -> Self {
        self.extensions.insert(extension.into(), kind);
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
