# Get started

## Following are the steps to use sbolt. See: [direct-compilation CLI example](../../rust/examples/cli_standalone/)

### 1. add sbolt crate and add sbolt crate to `[build-dependencies]`

```shell
cargo add sbolt
cargo add sbolt --build
```

### 2. create views and view models

For a standalone application, keep templates and their models under `src`:

```
$:.
│   build.rs
│   Cargo.toml
│
└───src
    │   main.rs
    │
    ├───view_models
    │       mod.rs
    │
    └───views
        │   default.rshtml
        │
        └───sub
                index.rshtml
```

`src/view_models/mod.rs`:

```rust
pub struct IndexModel {
    pub name: String,
    pub age: i32,
    pub msg: String,
}
```

`src/views/sub/index.rshtml`:
```rust
@model crate::view_models::IndexModel;
<html>
    <head>
        <title>Welcome</title>
    </head>
    <body>
        <div>@(model.msg) - from @(model.name)(@(model.age))</div>
    </body>
</html>
```

### 3. build.rs:

```rust
// `cargo build -vv` to show output.
fn main() {
    let mod_name = format!("{}_views", std::env!("CARGO_PKG_NAME"));
    let option = sbolt::codegen::CompilerOptions::default()
        .with_source_dir("src/views")
        .with_mod_name(&mod_name);
    let compiler = sbolt::codegen::Compiler::new(option);
    compiler.compile();
}
```

### 4. main.rs: include generated views and render view.

```rust
// Import the generated view modules.
sbolt::include_views!();

mod view_models;

fn main() {
    let mut context = sbolt::context!(view_models::IndexModel {
        name: "sbolt".into(),
        age: 1,
        msg: "Welcome!".into(),
    });
    let (output, _) = cli_views::render("views/sub/index", None, &mut context)
        .unwrap_or_else(|e| {
            eprintln!("Error: {e:?}");
            std::process::exit(1);
        });
    println!("{output}");
}
```

The [shared-crate CLI](../../rust/examples/cli/) and web examples keep their common templates in
[shared/example_views/views](../../rust/examples/shared/example_views/views) and model definitions in
[shared/example_views/src/models.rs](../../rust/examples/shared/example_views/src/models.rs).
The shared models and templates form one library crate. Each example adds it as a Cargo path
dependency, so no `#[path]` module declaration is needed.
The [example_views crate](../../rust/examples/shared/example_views) compiles
the common templates in its own build script and exposes `example_views::render`
and `example_views::models`.
These examples depend on this crate instead of maintaining their own template
build script or calling `sbolt::include_views!()`. Framework-specific handlers
and response adapters remain in the web examples.
These shared templates
refer to `example_views::models::IndexModel` or `example_views::models::WelcomeModel`; the standalone
layout above uses a local `crate::view_models` module instead.
Data-driven templates retrieve
`Arc<Model>` from the `"model"` context key. Static templates need no model.
`sbolt::context!(value)` wraps the value in `Arc` and stores it under `"model"`.
Explicit entries such as `sbolt::context!(model: model_arc, layout: layout_arc)`
store values as supplied; use these when models are already wrapped in `Arc`.

For the original direct-compilation style, [cli_standalone](../../rust/examples/cli_standalone/)
uses its own [build.rs](../../rust/examples/cli_standalone/build.rs) and calls
`sbolt::include_views!()` in its application. It reuses the shared templates and
imports model types from `example_views`, but renders through its own generated module.
Its dependency on the merged crate also builds that crate's shared renderer.

### 5. run

```sh
$>cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/cli`
<html>
    <head>
        <title>Welcome</title>
    </head>
    <body>
        <div>Welcome! - from sbolt(1)</div>
    </body>
</html>
```

Run either checked-in CLI example from [rust/examples](../../rust/examples/):

```shell
cargo run -p cli
cargo run -p cli-standalone
```

## Use in web applications

It's very simple to use sbolt in your web applications with popular `rust web frameworks`. Please see examples:
* [Use with Actix-web](../../rust/examples/web/actix)
* [Use with Axum](../../rust/examples/web/axum)
* [Use with Rocket](../../rust/examples/web/rocket)

Templates in the same folder can share a name when their kinds differ:
`welcome.rshtml`, `welcome.rsjson`, and `welcome.rstxt`. Select the representation
with `render(path, kind, context)`, which accepts `Option<Kind>` and returns
`RenderResult<(String, Kind)>`. `None` defaults to HTML inside the generated
renderer; the returned kind is always the resolved representation:

```rust
let (output, kind) = example_views::render(
    "views/welcome",
    Some(sbolt::types::template::Kind::KJSON),
    &mut context,
)?;
```

The framework-independent preference resolver is always available without
enabling a feature:

```toml
[dependencies]
sbolt = "0.0.2"

[build-dependencies]
sbolt = "0.0.2"
```

Each web example owns a local `ResponseFormat` wrapper around `Option<Kind>` and implements its
framework's request guard/extractor traits for it. Handlers receive
`format: ResponseFormat` and pass `format.into()` to `render`;
the renderer handles the HTML default. The wrapper satisfies Rust's orphan
rules without adding web-framework dependencies to sbolt.

The extractors support quality weights and wildcards and default to HTML when
`Accept` is absent. Malformed or unsupported preferences produce a
`ResponseFormat` containing `None`, so these examples also fall back to HTML in
those cases. Unexpected extraction errors still return HTTP 500. For strict
validation, call `resolve_kind` directly and map its invalid-input/not-found
errors to HTTP 400/406. The responders use the selected `Kind`
to set `Content-Type` and `Vary: Accept`. Missing templates and layouts remain
rendering failures (HTTP 500), without a cross-kind fallback.

The responders accept the render result through `.into()` and infer the content
type from its returned kind. The example responders add `Vary: Accept` by default,
including on rendering errors, so welcome handlers simply use `result.into()`.
This remains a web-layer policy, not a property of the renderer.

The example extractors share
`sbolt::resolve_kind(preferences)`. Request extraction,
response headers, and framework dependencies remain entirely in the examples.

The Actix-web, Axum, and Rocket examples negotiate HTML, JSON, or plain text at `/` using the request's
`Accept` header, including quality weights and wildcards. With no header it
defaults to HTML; invalid or unsupported formats also fall back to HTML. Responses set
the matching `Content-Type` and `Vary: Accept`. For example:

```shell
curl -H "Accept: application/json" http://localhost:8000/
curl -H "Accept: text/plain" http://localhost:8000/
```

The single `GET /` handler receives the selected template kind and renders it using
`render`. JSON and text requests render
[welcome.rsjson](../../rust/examples/shared/example_views/views/welcome.rsjson) and
[welcome.rstxt](../../rust/examples/shared/example_views/views/welcome.rstxt). Setting the response
content type does not validate JSON or escape dynamic values.

All three welcome representations declare `example_views::models::WelcomeModel`.
Each web handler stores an `Arc<WelcomeModel>` under `"model"`, so HTML, JSON, and
text read the same model field without individual data-store lookups.

---
### [Next: syntax](./syntax.md)

