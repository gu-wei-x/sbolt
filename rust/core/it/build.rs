fn main() {
    let option = disguise::codegen::CompilerOptions::default()
        .with_optimize(true)
        .with_source_dir("views")
        .with_mod_name("lib_views");
    let compiler = disguise::codegen::Compiler::new(option);
    compiler.compile();
}
