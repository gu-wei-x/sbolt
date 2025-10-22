use crate::{codegen::consts, types::template};
use std::collections::HashMap;

pub struct CompilerOptions {
    extensions: HashMap<String, template::Kind>,
    mod_name: String,
    need_optimization: bool,
    out_dir: Option<String>,
    source_dirs: Vec<String>,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        let options = CompilerOptions {
            extensions: HashMap::<String, template::Kind>::new(),
            mod_name: String::from(consts::TEMP_GENERATED_DIR),
            need_optimization: false,
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
        // todo: uncomment to turn on in release build.
        /*if cfg!(not(debug_assertions)) {
            let options = options.with_optimization(true);
            return options;
        } else {
            options
        }*/
    }
}

impl CompilerOptions {
    pub fn extensions(&self) -> &HashMap<String, template::Kind> {
        &self.extensions
    }

    pub fn with_extension(mut self, extension: &str, kind: template::Kind) -> Self {
        self.extensions.insert(extension.into(), kind);
        self
    }

    pub fn mod_name(&self) -> &String {
        &self.mod_name
    }

    pub fn with_mod_name(mut self, mod_name: &str) -> Self {
        self.mod_name = mod_name.into();
        self
    }

    pub fn need_optimization(&self) -> bool {
        self.need_optimization
    }

    pub fn with_optimization(mut self, need_optimization: bool) -> Self {
        self.need_optimization = need_optimization;
        self
    }

    pub fn source_dirs(&self) -> &[String] {
        &self.source_dirs
    }

    pub fn with_source_dir(mut self, source_dir: &str) -> Self {
        self.source_dirs.push(source_dir.into());
        self
    }

    pub fn out_dir(&self) -> &Option<String> {
        &self.out_dir
    }

    pub fn with_out_dir(mut self, out_dir: &str) -> Self {
        self.out_dir = Some(out_dir.into());
        self
    }
}
