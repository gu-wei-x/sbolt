fn main() {
    let option = sbolt::codegen::CompilerOptions::default()
        .with_optimize(true)
        .with_source_dir("views")
        .with_mod_name("bench_views");
    let compiler = sbolt::codegen::Compiler::new(option);
    compiler.compile();
}
