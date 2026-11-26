fn main() {
    sbolt::codegen::Compiler::new(
        sbolt::codegen::CompilerOptions::default()
            .with_source_dir("src/views")
            .with_mod_name("shared_views"),
    )
    .compile();
}
