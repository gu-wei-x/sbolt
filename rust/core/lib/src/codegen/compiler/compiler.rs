use crate::codegen::CompileResult;
use crate::codegen::CompilerOptions;
use crate::codegen::consts;
use crate::codegen::module::Module;
use crate::codegen::registry;
use crate::types::result;
use crate::utils;
use std::env;
use std::path::PathBuf;

pub struct Compiler {
    pub(crate) options: CompilerOptions,
}

impl Compiler {
    pub fn new(options: CompilerOptions) -> Self {
        Compiler { options }
    }

    // called by build script to process view templates.
    pub fn compile(&self) -> result::Result<CompileResult> {
        // TODO: implement compilation logic, incremental build, multiple tasks to improve performance, etc.
        let target_dir = if let Some(dir) = &self.options.out_dir {
            dir
        } else {
            &PathBuf::from(env::var(consts::OUT_DIR_ENV_NAME).unwrap())
                .join(consts::TEMP_GENERATED_DIR)
                .to_str()
                .unwrap()
                .to_string()
        };

        let mut compiler_result = CompileResult::default();
        compiler_result.add_mod(consts::TEMPLATES_MAP_FILE_NAME);
        for dir in &self.options.source_dirs {
            Module::new(PathBuf::from(dir), PathBuf::from(target_dir), None)
                .process(&self.options)?
                .merge_into(&mut compiler_result);
            match utils::fs::get_dir_name(dir) {
                Some(name) => compiler_result.add_mod(&name),
                _ => {}
            }
        }

        // Generate the view map.
        let view_mapping = compiler_result.view_name_mapping();
        let view_map_file_path = PathBuf::from(target_dir).join(format!(
            "{}{}",
            consts::TEMPLATES_MAP_FILE_NAME,
            consts::RS_FILE_EXTENSION
        ));

        _ = registry::generate_registry(&view_map_file_path, &self.options.mod_name, view_mapping);

        let root_mod_file_path = PathBuf::from(target_dir).join(consts::TEMPLATES_MOD_FILE_NAME);
        let root_mod_ts =
            Module::generate_root_mod_ts(&self.options.mod_name, compiler_result.mods());
        _ = utils::fs::generate_code_with_content(&root_mod_file_path, &root_mod_ts);

        // Tell cargo to rerun the build script if any of the source directories change.
        for dir in &self.options.source_dirs {
            println!("cargo:rerun-if-changed={}", dir);
        }

        // Set environment variable for the generated mod file path.
        println!(
            "cargo:rustc-env={}={}",
            consts::TEMPLATES_FILES_ENV,
            root_mod_file_path.display()
        );

        Ok(compiler_result)
    }
}
