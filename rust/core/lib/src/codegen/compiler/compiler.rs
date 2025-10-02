use crate::codegen::CompileResult;
use crate::codegen::CompilerOptions;
use crate::codegen::compiler::fsutil;
use crate::codegen::compiler::module::Module;
use crate::codegen::compiler::registry;
use crate::codegen::consts;
use crate::types::result;
use std::env;
use std::path::PathBuf;

pub struct Compiler {
    pub(crate) options: CompilerOptions,
}

impl Compiler {
    pub fn new(options: CompilerOptions) -> Self {
        Compiler { options }
    }

    #[allow(unused_doc_comments)]
    pub fn compile(&self) -> CompileResult {
        match self.process() {
            Ok(result) => result,
            Err(err) => {
                /**
                 * cargo:error= is not impled for build scripts to report issues
                 * change in future with cargo:error format to let ide highlight issue.
                 */
                eprintln!("\x1b[0;31merror\x1b[0m:{err:#}");
                std::process::exit(1);
            }
        }
    }
}

impl Compiler {
    // called by build script to process view templates.
    fn process(&self) -> result::Result<CompileResult> {
        // TODO: implement compilation logic, incremental build, multiple tasks to improve performance, etc.
        let target_dir = if let Some(dir) = &self.options.out_dir {
            dir
        } else {
            &PathBuf::from(env::var(consts::OUT_DIR_ENV_NAME)?)
                .join(consts::TEMP_GENERATED_DIR)
                .to_str()
                .ok_or("Failed to create temp dir for procossing views")?
                .to_string()
        };

        let mut compiler_result = CompileResult::default();
        compiler_result.add_mod(consts::TEMPLATES_MAP_FILE_NAME);
        for dir in &self.options.source_dirs {
            Module::new(PathBuf::from(dir), PathBuf::from(target_dir), None)
                .process(&self.options)?
                .merge_into(&mut compiler_result);
            compiler_result.add_mod(
                &fsutil::get_dir_name(dir)
                    .ok_or(format!("Unable to generate mod name for dir: {dir}"))?,
            );
        }

        // Generate the view map.
        let view_mapping = compiler_result.view_name_mapping();
        let view_map_file_path = PathBuf::from(target_dir).join(format!(
            "{}{}",
            consts::TEMPLATES_MAP_FILE_NAME,
            consts::RS_FILE_EXTENSION
        ));

        registry::generate_registry(&view_map_file_path, &self.options.mod_name, view_mapping)?;

        let root_mod_file_path = PathBuf::from(target_dir).join(consts::TEMPLATES_MOD_FILE_NAME);
        let root_mod_ts =
            Module::generate_root_mod_ts(&self.options.mod_name, compiler_result.mods());
        fsutil::write_code_to_file(&root_mod_file_path, &root_mod_ts)?;

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
