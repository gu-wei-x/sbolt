# Keywords

sbolt recognizes six keywords after `@`. Other interpolated values use Rust
expressions; see [Syntax](./syntax.md).

| Keyword | Example | Purpose |
| --- | --- | --- |
| `layout` | `@layout layout;` | Wrap this template in another template |
| `model` | `@model crate::view_models::IndexModel;` | Retrieve the template's typed model |
| `use` | `@use std::sync::Arc;` | Add a Rust import |
| `section` | `@section title { <title>Home</title> }` | Store content under a section name |
| `render` | `@render(title)` | Insert a section from the current context |
| `view` | `@view("views/card", sbolt::context!())` | Render a separate template inline |

## `@layout`

```text
@layout layout;
```

The argument is an unquoted template path, without an extension. An ordinary
path searches the current template directory and then its ancestors. `/` makes
the path absolute within the view registry; `~/` restricts it to the current
template directory.

The layout must match the page's HTML, JSON, or text kind. Its output replaces
the page output; the page body is available through `@render`.

See [Layout lookup](./layout.md#layout-lookup).

## `@model`

```text
@model crate::view_models::IndexModel;
```

This retrieves `Arc<IndexModel>` under `"model"` and exposes it as `model`.
To select another context key:

```text
@model layout: crate::view_models::LayoutModel;
```

The key is an unquoted identifier. The template variable is still `model`.
Only one model declaration is allowed per template, at the top level.

See [Typed models](./syntax.md#typed-models).

## `@use`

```text
@use std::sync::Arc;
```

The argument is a Rust import path. Imports can make a model declaration or
expression shorter:

```text
@use crate::view_models::IndexModel;
@model IndexModel;
<p>@(model.name)</p>
```

The `layout`, `model`, and `use` directives end at a semicolon or newline.
Prefer a semicolon to make their boundaries explicit. A model type can also
span lines while its Rust type syntax is incomplete.

## `@section`

```text
@section title {
    <title>Home</title>
}
```

The section name is an unquoted identifier, followed by a content block.
The content can include interpolation and Rust code. Defining a section stores
its output in the context rather than inserting it at that position.
Repeated names append content in definition order.

See [Sections](./layout.md#sections).

## `@render`

| Form | Behavior |
| --- | --- |
| `@render` or `@render()` | Render the default section; fail if it is absent |
| `@render(title)` | Render a required named section |
| `@render(title, true)` | Same required-section behavior |
| `@render(title, false)` | Render it if present; otherwise emit nothing |

The section name is an unquoted identifier, not a Rust string expression.
The optional second argument must be the literal `true` or `false`.

`@render` reads the current context. It does not look up another template.

## `@view`

```text
@view("views/card", sbolt::context!())
@view("views/data", sbolt::context!(), sbolt::types::template::Kind::KJSON)
```

Arguments are Rust expressions:

1. A registered template name as `&str`.
2. An explicit owned or mutable-borrowed context.
3. Optionally, a `Kind` value to override the caller's kind.

Two or three arguments are accepted, with an optional trailing comma. They are
evaluated once, from left to right. Without the third argument, the child uses
the caller's kind. There is no fallback to a different kind.

The child applies its own layout and inserts its output directly. Missing
templates and rendering failures propagate to the caller.

See [Custom views](./syntax.md#custom-views) for context sharing and output safety.

## Literal keywords and escapes

Use `@(model)` to interpolate a value named `model` instead of starting a model
declaration. Explicit `@(...)` expressions also avoid other keyword conflicts.
Use `@@` for a literal `@`, and `@* comment *@` for a template comment.

[Get started](./get_started.md) | [Syntax](./syntax.md) | [Layouts](./layout.md)
