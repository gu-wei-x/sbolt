fn main() {
    let option = sbolt::codegen::CompilerOptions::default()
        .with_optimize(true)
        .with_source_dir("src/views")
        .with_mod_name("actix_web_example_views");
    let compiler = sbolt::codegen::Compiler::new(option);
    compiler.compile();
}
