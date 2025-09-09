// `cargo build -vv` to show output.
use disguise;
use std::env;
use std::path::PathBuf;

fn main() {
    // 1. generate intermediate files for view templates
    // 2. generate a list file which contains all generated intermediate file paths.
    // 3. use include! macro to include list file in app main.rs
    let out_dir = env::var("OUT_DIR").unwrap();
    let view_dir = PathBuf::from("src/views");
    let temp_dir = PathBuf::from(out_dir).join("views");

    let view_dir = view_dir.canonicalize().unwrap();
    println!("cargo:rerun-if-changed={}", view_dir.display());

    let crate_name = format!("{}_views", env!("CARGO_PKG_NAME"));
    let generated_index = disguise::process_views(view_dir, temp_dir, &crate_name);
    println!("cargo:rustc-env=VIEW_FILES={}", generated_index.display());
}
