pub(crate) mod consts;
mod views;

use crate::utils;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

// called by build.rs to process view templates and generate intermediate files.
// returns the path to the generated mod.rs file which contains all view modules.
// mod.rs path will be used in the include! macro in the main.rs file of the application.
// TODO: returns result instead of panicking on errors.
pub fn process_views(view_dir: &str, crate_name: &str) -> PathBuf {
    let view_dir = PathBuf::from(view_dir);
    let view_dir = view_dir.canonicalize().unwrap();
    if (view_dir.exists() == false) || (view_dir.is_dir() == false) {
        panic!("view folder does not exist or is not a folder.");
    }

    let mut template_files: Vec<PathBuf> = vec![];
    utils::fs::get_files_with_extension(&view_dir, &["rshtml".into()], &mut template_files);
    if template_files.is_empty() {
        panic!("no view files found");
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let temp_dir = PathBuf::from(out_dir).join("views");

    _ = std::fs::create_dir_all(temp_dir.clone());
    let mut imported_mods: Vec<String> = vec![consts::TEMPLATES_MAP_FILE_NAME.to_string()];
    let mut view_name_mapping: HashMap<String, String> = HashMap::new();
    for template_file in &template_files {
        let prefix = &format!("_{}_generated_view_", crate_name);
        let name =
            utils::fs::path_to_name(&view_dir, template_file, prefix, consts::RS_FILE_EXTENSION);
        let mod_name = name.strip_suffix(consts::RS_FILE_EXTENSION).unwrap();
        imported_mods.push(mod_name.to_string());

        let generated_file_path: PathBuf = temp_dir.join(&name);
        _ = views::generate_view_content(
            &generated_file_path,
            &view_dir,
            &template_file,
            &mut view_name_mapping,
        );
    }

    let mod_file_path = temp_dir.join(consts::TEMPLATES_MOD_FILE_NAME);
    _ = views::generate_mod_content(
        &mod_file_path,
        crate_name,
        &imported_mods,
        &view_name_mapping,
    );

    // Generate the view map.
    let view_map_file_path = temp_dir.join(format!("{}.rs", consts::TEMPLATES_MAP_FILE_NAME));
    _ = views::generate_view_map(&view_map_file_path, crate_name, &view_name_mapping);

    // Tell cargo to rerun the build script if the view directory changes.
    println!("cargo:rerun-if-changed={}", view_dir.display());
    println!(
        "cargo:rustc-env={}={}",
        consts::TEMPLATES_FILES_ENV,
        mod_file_path.display()
    );
    mod_file_path
}
