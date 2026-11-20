fn main() {
    let option = sbolt::codegen::CompilerOptions::default()
        .with_source_dir("../shared/example_views/views")
        .with_mod_name("cli_standalone_views");
    let compiler = sbolt::codegen::Compiler::new(option);
    compiler.compile();
}
