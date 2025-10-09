fn main() {
    let option = disguise::codegen::CompilerOptions::default()
        .with_optimize(true)
        .with_source_dir("src/views")
        .with_mod_name("axum_example_views");
    let compiler = disguise::codegen::Compiler::new(option);
    compiler.compile();
}
