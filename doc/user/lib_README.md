# sbolt

sbolt compiles templates into Rust during your Cargo build. Templates combine
literal content with Rust expressions and code, using Razor-inspired `@` syntax.
At runtime, your application calls the generated renderer rather than parsing
template source.

## What it supports

- HTML, JSON, and plain-text templates.
- Typed models with ordinary Rust field access.
- Named context values for templates without a model.
- Layouts, named sections, and reusable child views.
- Resolve-once rendering for repeatedly used templates.
- Streaming value formatting and output-capacity hints.

Template syntax and generated Rust are checked during the build. Missing
templates, missing sections, and incorrectly typed context values can still
produce runtime errors.

## A template with a model

```text
@model crate::view_models::WelcomeModel;
<p>Welcome, @(model.name)!</p>
```

The model is a normal Rust type:

```rust
pub struct WelcomeModel {
    pub name: String,
}
```

Create the context with `sbolt::context!(model_value)`. The macro wraps the model
in `Arc` and stores it under `"model"`. Explicit named entries remain available,
for example `sbolt::context!(name: String::from("Rocket"))`.

See [Get started](./get_started.md) for a complete project, including its build
script and rendering call.

## Output safety

Interpolation writes values **without automatic HTML or JSON escaping**. A
template's extension selects its representation; it does not make interpolated
values safe for that format. Escape or serialize untrusted values appropriately
before rendering. Dynamic JSON output is not validated automatically.

## Documentation

| Guide | Contents |
| --- | --- |
| [Get started](./get_started.md) | Project setup, contexts, rendering APIs, compiler options, and examples |
| [Syntax](./syntax.md) | Expressions, Rust code, models, formatting, and custom views |
| [Keywords](./keywords.md) | The six supported template keywords and their arguments |
| [Layouts and sections](./layout.md) | Layout resolution, section rendering, and independent layout models |

## Repository examples

- [CLI using the shared view crate](../../rust/examples/cli)
- [CLI compiling templates directly](../../rust/examples/cli_standalone)
- [Actix Web](../../rust/examples/web/actix), [Axum](../../rust/examples/web/axum),
  and [Rocket](../../rust/examples/web/rocket)

The [shared example crate](../../rust/examples/shared/example_views) owns both
the model definitions and the compiled templates. HTTP dependencies and
response handling remain in the web examples.
