// `cargo build -vv` to show output.
fn main() {
    let mod_name = format!("{}_views", std::env!("CARGO_PKG_NAME"));
    let option = sbolt::codegen::CompilerOptions::default()
        .with_source_dir("../views")
        .with_mod_name(&mod_name);
    let compiler = sbolt::codegen::Compiler::new(option);
    compiler.compile();
}
