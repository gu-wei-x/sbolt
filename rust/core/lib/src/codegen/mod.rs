pub(crate) mod views;

use std::collections::HashMap;
use std::path::PathBuf;

use crate::utils;

pub fn process_views(view_dir: PathBuf, temp_dir: PathBuf, crate_name: &str) -> PathBuf {
    if (view_dir.exists() == false) || (view_dir.is_dir() == false) {
        panic!("view folder does not exist or is not a folder.");
    }

    let mut template_files: Vec<PathBuf> = vec![];
    utils::fs::get_files_with_extension(&view_dir, &["rshtml".into()], &mut template_files);
    if template_files.is_empty() {
        panic!("no view files found");
    }

    _ = std::fs::create_dir_all(temp_dir.clone());
    let mut imported_mods: Vec<String> = vec![];
    let mut view_name_mapping: HashMap<String, String> = HashMap::new();
    for template_file in &template_files {
        let prefix = &format!("_{}_generated_view_", crate_name);
        let name = utils::fs::path_to_name(&view_dir, template_file, prefix, ".rs");
        let mod_name = name.strip_suffix(".rs").unwrap();
        imported_mods.push(mod_name.to_string());

        let generated_file_path: PathBuf = temp_dir.join(&name);
        _ = views::generate_view_content(
            &generated_file_path,
            &view_dir,
            &template_file,
            &mut view_name_mapping,
        );
    }

    let mod_file_path = temp_dir.join("mod.rs");
    _ = views::generate_mod_content(
        &mod_file_path,
        crate_name,
        &imported_mods,
        &view_name_mapping,
    );

    mod_file_path
}
