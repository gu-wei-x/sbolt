// `cargo build -vv` to show output.
fn main() {
    let mod_name = format!("{}_views", std::env!("CARGO_PKG_NAME"));
    let option = disguise::codegen::CompilerOptions::default()
        .with_optimize(true)
        .with_source_dir("src/views")
        .with_mod_name(&mod_name);
    let compiler = disguise::codegen::Compiler::new(option);
    compiler.compile();
}
