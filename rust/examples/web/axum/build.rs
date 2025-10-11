fn main() {
    let option = sbolt::codegen::CompilerOptions::default()
        .with_optimize(true)
        .with_source_dir("src/views")
        .with_mod_name("axum_example_views");
    let compiler = sbolt::codegen::Compiler::new(option);
    compiler.compile();
}
