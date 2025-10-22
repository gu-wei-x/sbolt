fn main() {
    let option = sbolt::codegen::CompilerOptions::default()
        // turn optimization on when feature is ready.
        .with_optimization(true)
        .with_source_dir("views")
        .with_mod_name("bench_views");
    let compiler = sbolt::codegen::Compiler::new(option);
    compiler.compile();
}
