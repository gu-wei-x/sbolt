<p align="center">
  <img src="./doc/user/images/sbolt.png" alt="sbolt icon" width="144" height="144">
</p>

<h1 align="center">sbolt</h1>

<p align="center">
  <a href="https://github.com/gu-wei-x/sbolt"><img alt="github" src="https://img.shields.io/badge/github-guweix/sbolt-8da0cb?style=for-the-badge&amp;labelColor=555555&amp;logo=github" height="20"></a>
  <a href="https://crates.io/crates/sbolt"><img alt="crates.io" src="https://img.shields.io/crates/v/sbolt.svg?style=for-the-badge&amp;color=fc8d62&amp;logo=rust" height="20"></a>
  <a href="https://docs.rs/sbolt"><img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-sbolt-66c2a5?style=for-the-badge&amp;labelColor=555555&amp;logo=docs.rs" height="20"></a>
  <a href="https://github.com/gu-wei-x/sbolt/actions?query=branch%3Amain"><img alt="build status" src="https://img.shields.io/github/actions/workflow/status/gu-wei-x/sbolt/ci.yml?branch=main&amp;style=for-the-badge" height="20"></a>
</p>

sbolt is a Rust template engine that compiles templates during your Cargo build.
Its Razor-inspired `@` syntax combines literal content with Rust expressions,
typed models, layouts, and reusable views.

## Why sbolt?

- **Build-time checks:** template syntax and generated Rust are checked before deployment.
- **Compiled rendering:** no template-source parsing at runtime.
- **Flexible output:** HTML, JSON, and plain text, with matching-kind layouts.
- **Efficient formatting:** stream values into output buffers and reuse resolved templates, See the [benchmarks](./rust/benchmark) for measured comparisons.

## Quick start

Add sbolt for both runtime use and build-time compilation:

```shell
cargo add sbolt
cargo add sbolt --build
```

Create `src/views/welcome.rshtml`:

```text
@model crate::WelcomeModel;
<p>Welcome, @(model.name)!</p>
```

Compile the templates in `build.rs`:

```rust
fn main() {
    let option = sbolt::codegen::CompilerOptions::default()
        .with_source_dir("src/views")
        .with_mod_name("compiled_views");
    sbolt::codegen::Compiler::new(option).compile();
}
```

Define the model and render from `src/main.rs`:

```rust
sbolt::include_views!();

pub struct WelcomeModel {
    pub name: String,
}

fn main() -> sbolt::types::result::RenderResult<()> {
    let mut context = sbolt::context!(WelcomeModel {
        name: "sbolt".into(),
    });
    let (output, _) = compiled_views::render("views/welcome", None, &mut context)?;
    println!("{output}");
    Ok(())
}
```

Run with `cargo run`.


## Documentation

- [Get started](./doc/user/get_started.md): Setup and usage.
- [Development](./doc/dev/README.md): Architecture and roadmap.

## License

This crate is licensed under [LICENSE](./LICENSE)

