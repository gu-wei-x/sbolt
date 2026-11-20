fn main() {
    let no_layout_dir =
        std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("no_layout_views");
    let no_layout_options = sbolt::codegen::CompilerOptions::default()
        .with_source_dir("views_no_layout")
        .with_out_dir(no_layout_dir.to_str().unwrap())
        .with_mod_name("no_layout_views");
    sbolt::codegen::Compiler::new(no_layout_options).compile();
    println!(
        "cargo:rustc-env=NO_LAYOUT_TEMPLATES_FILES={}",
        no_layout_dir.join("mod.rs").display()
    );

    let option = sbolt::codegen::CompilerOptions::default()
        .with_source_dir("../views")
        .with_mod_name("lib_it_no_op_views");
    let compiler = sbolt::codegen::Compiler::new(option);
    compiler.compile();
}
