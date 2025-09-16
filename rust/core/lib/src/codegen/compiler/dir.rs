use crate::codegen::compiler;
use crate::codegen::consts;
use crate::codegen::views;
use crate::utils;
use std::{fs, path::Path};

pub(crate) fn process_dir<P: AsRef<Path>>(
    source_dir: &P,
    target_dir: &P,
    namespace: &Option<String>,
    compiler_options: &compiler::CompilerOptions,
    compiler_result: &mut compiler::CompileResult,
) {
    // validate source_dir exists and is a directory.
    if !source_dir.as_ref().is_dir() {
        return;
    }

    // process.
    let dir_name = &utils::fs::get_dir_name(source_dir).unwrap();
    let target_dir = &utils::fs::create_target_dir(&target_dir.as_ref().to_path_buf(), dir_name);
    if let Ok(read_dir) = fs::read_dir(source_dir) {
        let mut mods = Vec::new();
        let full_name = &Some(utils::name::create_full_name(namespace, dir_name));
        for entry in read_dir.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    if let Some(dir_name) = entry.file_name().to_str() {
                        process_dir(
                            &entry.path(),
                            target_dir,
                            full_name,
                            compiler_options,
                            compiler_result,
                        );
                        mods.push(dir_name.to_string());
                    }
                } else if meta.is_file() {
                    if utils::fs::match_file_with_ext(&entry.path(), &compiler_options.extensions) {
                        process_template_file(
                            &entry.path(),
                            target_dir,
                            full_name,
                            &mut mods,
                            compiler_result,
                        );
                    }
                }
            }
        }

        views::generate_sub_mod_file(&target_dir.join(consts::TEMPLATES_MOD_FILE_NAME), &mods);
    }
}

fn process_template_file<P: AsRef<Path>>(
    template_file: &P,
    target_dir: &P,
    namespace: &Option<String>,
    mods: &mut Vec<String>,
    compiler_result: &mut compiler::CompileResult,
) {
    if let Some(file_stem) = utils::fs::get_file_name(template_file) {
        let file_name = format!("{}{}", file_stem, consts::RS_FILE_EXTENSION);
        let file = target_dir.as_ref().join(file_name);
        _ = views::generate_view_content(
            &file,
            &template_file.as_ref().to_path_buf(),
            &file_stem,
            namespace,
            compiler_result,
        );
        mods.push(file_stem.to_string());
    }
}
