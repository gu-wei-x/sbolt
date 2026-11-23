# Get started

This guide first builds a standalone application, then explains the shared-crate
approach used by the repository examples. Use a current Rust toolchain that
supports the 2024 edition.

## 1. Add the dependency

Add sbolt for both runtime rendering and build-time template compilation:

```shell
cargo add sbolt
cargo add sbolt --build
```

The optional `pretty` feature formats generated Rust source; it does not format
the rendered HTML, JSON, or text.

## 2. Create the model and template

Keep the model in a separate module:

```text
my_app/
|-- Cargo.toml
|-- build.rs
`-- src/
    |-- main.rs
    |-- view_models/
    |   `-- mod.rs
    `-- views/
        `-- index.rshtml
```

`src/view_models/mod.rs`:

```rust
pub struct IndexModel {
    pub name: String,
    pub age: i32,
    pub msg: String,
}
```

`src/views/index.rshtml`:

```text
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

The declaration retrieves an `Arc<IndexModel>` from the context. Rust field
access dereferences the `Arc` automatically. Model types and fields must be
accessible from the generated template modules.

## 3. Compile templates in `build.rs`

```rust
fn main() {
    let option = sbolt::codegen::CompilerOptions::default()
        .with_source_dir("src/views")
        .with_mod_name("compiled_views");
    let compiler = sbolt::codegen::Compiler::new(option);
    compiler.compile();
}
```

`with_mod_name` selects a Rust module identifier independently of the Cargo
package name. The compiler writes generated files under Cargo's `OUT_DIR`,
sets `TEMPLATES_FILES`, and asks Cargo to rerun when a source directory changes.
Template compilation errors stop the build.

## 4. Render from `src/main.rs`

```rust
sbolt::include_views!();

mod view_models;

fn main() {
    let mut context = sbolt::context!(view_models::IndexModel {
        name: "sbolt".into(),
        age: 1,
        msg: "Welcome!".into(),
    });
    let (output, _) = compiled_views::render("views/index", None, &mut context)
        .unwrap_or_else(|e| {
            eprintln!("Error: {e:?}");
            std::process::exit(1);
        });
    println!("{output}");
}
```

The template name is `"views/index"`: it includes the source directory's final
component but omits the extension. `None` selects HTML. The result contains the
output and the selected template kind.

## 5. Run

```shell
cargo run
```

The rendered page contains `<div>Welcome! - from sbolt(1)</div>`.

## Contexts

### A single typed model

`sbolt::context!(value)` is shorthand for
`sbolt::context!(model: std::sync::Arc::new(value))`.
It evaluates the value once and accepts an optional trailing comma.

For an existing `Arc`, use an explicit entry to avoid wrapping it again:

```rust
let model = std::sync::Arc::new(view_models::IndexModel {
    name: "sbolt".into(),
    age: 1,
    msg: "Welcome!".into(),
});
let mut context = sbolt::context!(model: model);
```

Creating an `Arc` allocates once for the model. Rendering clones the `Arc`,
not the model or its string fields. It does not allocate an `Arc` per field.

### Named values without a model

Templates do not have to declare a typed model:

```rust
let mut context = sbolt::context!(
    name: String::from("sbolt"),
    age: 1i32
);
```

Read these values with `context.get_data::<T>("key")` in the template. Named
entries are stored as supplied, without automatic `Arc` wrapping. `get_data`
returns `None` for a missing key or a different stored type.

Use `String::from(...)` or `.to_string()` when a named entry needs a `String`:
unlike a typed struct field, an unconstrained `.into()` cannot establish its
type. A string literal stored directly is an `&'static str`, not a `String`.

All context values must be `Send + Sync + 'static`. Use `sbolt::context!()` for an
empty context. In `DefaultViewContext`, data keys are write-once: setting an
existing key keeps the original value. Prefer fresh contexts for new inputs.

## Rendering API

The examples below use the `compiled_views` module configured earlier:

| Call | Result | Purpose |
| --- | --- | --- |
| `compiled_views::render(name, kind, context)` | `RenderResult<(String, Kind)>` | Resolve a name and render |
| `compiled_views::resolve(name, kind)` | `RenderResult<KTemplate>` | Resolve an entry template once |
| `template.render(context)` | `RenderResult<String>` | Render a previously resolved template |

The `context` argument is mutable. The first two calls accept `Option<Kind>`;
`None` selects HTML, not whichever representation happens to exist.

```rust
let template = compiled_views::resolve("views/index", None)?;
let output = template.render(&mut context)?;
```

This fragment belongs in a function that returns a compatible `Result`.
Cache the resolved template outside a repeated rendering loop to avoid repeated
entry-name normalization and lookup. This does not cache output, context values,
layout resolution, or child-view lookup.

### Template kinds

| Extension | Kind | Representation |
| --- | --- | --- |
| `.rshtml` | `Kind::KHTML` | HTML |
| `.rsjson` | `Kind::KJSON` | JSON |
| `.rstxt` | `Kind::KTEXT` | Plain text |

Import `Kind` from `sbolt::types::template`. Templates may share a name when their
kinds differ. Missing names or missing requested kinds return rendering errors;
the renderer does not fall back to another kind.

### Capacity hints

Generated templates estimate static output size. For dynamic content or loops,
provide an estimate of the total output bytes:

```rust
context.set_capacity_hint(16 * 1024);
let (output, kind) = compiled_views::render("views/index", None, &mut context)?;
```

The estimate is a reservation hint, not a size limit. Rendering and layouts
sharing the context use the greater of their static estimate and the context
hint. Section buffers retain their own static estimates.

## Compiler options

`CompilerOptions::default()` registers the three extensions above, uses module
name `generated_views`, and sets whitespace optimization to
`cfg!(not(debug_assertions))` in sbolt: normally disabled in debug builds and
enabled in release builds.

| Builder | Effect |
| --- | --- |
| `with_source_dir(path)` | Add a template source directory; may be called more than once |
| `with_mod_name(name)` | Set the generated rendering module's Rust identifier |
| `with_extension("html", Kind::KHTML)` | Add or replace an extension mapping, without a leading dot |
| `with_optimization(bool)` | Override static-write merging and conservative whitespace compaction; defaults to `cfg!(not(debug_assertions))` in sbolt |
| `with_out_dir(path)` | Override the generated output directory |

Optimization can change whitespace where the compiler considers it safe under
default HTML whitespace/display rules. CSS overrides may make that whitespace
significant. Protected content, dynamic JSON, and unknown insertion contexts
are preserved. Fully static JSON is validated when optimization is enabled;
dynamic JSON is not validated. See [optimization details](../dev/optimization.md).

## Shared views and repository examples

The [example_views crate](../../rust/examples/shared/example_views) combines:

```text
shared/example_views/
|-- Cargo.toml
|-- build.rs
|-- src/
|   |-- lib.rs
|   `-- models.rs
`-- views/
    |-- default.rshtml
    |-- welcome.rshtml
    |-- welcome.rsjson
    |-- welcome.rstxt
    `-- sub/index.rshtml
```

The [shared CLI](../../rust/examples/cli) and web examples depend on this crate
and call `example_views::render`. They do not need their own template build
scripts or `include_views!()` calls. Models are exposed through
`example_views::models`; no `#[path]` module declarations are needed.

The shared crate's [library root](../../rust/examples/shared/example_views/src/lib.rs)
aliases itself as `example_views` so its template model paths work both inside
that crate and in the standalone example. Its public wrapper is needed because
the generated renderer is `pub(crate)` and cannot be publicly re-exported.
The generated module also provides `resolve`, but the example crate's public
wrapper currently exposes only named `render`.

The [standalone CLI](../../rust/examples/cli_standalone) instead compiles those
same template files with its own build script and uses `sbolt::include_views!()`.
It imports model types from `example_views` but renders through
`cli_standalone_views`. Its dependency on the combined crate still builds the
shared renderer too.

Run the CLI examples from [rust/examples](../../rust/examples):

```shell
cargo run -p cli
cargo run -p cli-standalone
```

## Web applications

The [Actix Web](../../rust/examples/web/actix), [Axum](../../rust/examples/web/axum),
and [Rocket](../../rust/examples/web/rocket) examples serve HTML, JSON, or text
at `/`. They keep request extraction, HTTP dependencies, and responders outside
the template crate.

`sbolt::resolve_kind(preferences)` selects a `Kind` from an HTTP `Accept` value.
It is available without a feature flag:

```rust
let kind = sbolt::resolve_kind(Some("text/html;q=0.5, application/json"))?;
assert_eq!(kind, sbolt::types::template::Kind::KJSON);
```

The resolver supports quality weights and wildcards. No header selects HTML.
Malformed preferences return `RuntimeError::InvalidInput`; preferences that
match no supported kind return `RuntimeError::NotFound`.

The examples intentionally map either error to `ResponseFormat(None)` and render
HTML. Unexpected extraction errors and rendering failures return HTTP 500.
For stricter behavior, map invalid preferences to HTTP 400 and unsupported
preferences to HTTP 406 in your own adapter.

Responders use the returned `Kind` to set `Content-Type` and add `Vary: Accept`,
including on rendering errors. These headers and fallback policies belong to
the examples, not the renderer.

From the examples workspace, start one web example:

```shell
cargo run -p axum-example
```

Use `actix-web-example` or `rocket-example` for the other frameworks. Run only
one at a time because all three examples use port 8000. In another terminal,
request its different representations:

```shell
curl -H "Accept: application/json" http://localhost:8000/
curl -H "Accept: text/plain" http://localhost:8000/
```

All three welcome templates use `example_views::models::WelcomeModel`. Selecting
JSON or setting a content-type header does not validate dynamic output or escape
its values. Apply appropriate escaping or serialization before rendering.

[Overview](./lib_README.md) | [Syntax](./syntax.md) | [Keywords](./keywords.md) | [Layouts](./layout.md)
