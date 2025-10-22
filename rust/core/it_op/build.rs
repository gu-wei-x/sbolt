fn main() {
    let option = sbolt::codegen::CompilerOptions::default()
        .with_optimization(true)
        .with_source_dir("views")
        .with_mod_name("lib_it_op_views");
    let compiler = sbolt::codegen::Compiler::new(option);
    compiler.compile();
}
