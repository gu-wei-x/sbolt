fn main() {
    let option = sbolt::codegen::CompilerOptions::default()
        .with_source_dir("../shared/views")
        .with_mod_name("cli_standalone_views");
    let compiler = sbolt::codegen::Compiler::new(option);
    compiler.compile();
}
