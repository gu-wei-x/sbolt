use crate::codegen::CompileResult;
use crate::codegen::CompilerOptions;
use crate::codegen::consts;
use crate::codegen::dir;
use crate::codegen::views;
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
    pub fn compile(&self) -> CompileResult {
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
            let top_mod_name = utils::fs::get_dir_name(dir).unwrap();
            compiler_result.add_mod(&top_mod_name);
            dir::process_dir(dir, target_dir, &None, &self.options, &mut compiler_result);
        }

        if compiler_result.is_success() {
            // Generate the view map.
            let view_mapping = compiler_result.view_name_mapping();
            let view_map_file_path = PathBuf::from(target_dir).join(format!(
                "{}{}",
                consts::TEMPLATES_MAP_FILE_NAME,
                consts::RS_FILE_EXTENSION
            ));
            _ = views::generate_view_types(
                &view_map_file_path,
                &self.options.mod_name,
                view_mapping,
            );
            let root_mod_file_path =
                PathBuf::from(target_dir).join(consts::TEMPLATES_MOD_FILE_NAME);
            _ = views::generate_root_mod_file(
                &root_mod_file_path,
                &self.options.mod_name,
                compiler_result.mods(),
            );

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
        }

        compiler_result
    }
}
