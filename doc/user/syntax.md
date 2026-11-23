# Syntax

Templates start in content mode. Literal content is written to the output.
`@` introduces a Rust expression, code block, or one of the
[six template keywords](./keywords.md).

The default extensions are `.rshtml` for HTML, `.rsjson` for JSON, and `.rstxt`
for text. The same Rust and directive syntax is available in all three kinds.

## Expressions

Use `@value` for a simple expression or `@(expression)` for an explicit boundary:

```text
@{
    let name = "sbolt";
    let count = 3;
}
<p>@name: @(count + 1)</p>
```

The paragraph contains `sbolt: 4`. Expressions are evaluated during rendering,
not during template compilation.

Prefer `@(...)` for field access, complex expressions, and values next to
punctuation. For example, `@(left)|@(right)` avoids interpreting `|` as part of a
Rust expression. Values must support the formatting described below.

## Rust code and content blocks

In content mode, `@{ ... }` contains Rust statements. Statements do not emit
output by themselves:

```text
@{
    let message = "Hello";
}
<p>@(message)</p>
```

Inside Rust code, another `@{ ... }` switches back to content. Use this for loops
and conditionals:

```text
<ul>
@{
    for name in ["Ada", "Lin"] {
        @{<li>@(name)</li>}
    }
}
</ul>
```

Variables follow ordinary Rust scoping, borrowing, and declaration order.
Templates generate synchronous render methods; `@` is not a separate
scripting language.

## Escapes and comments

Use `@@` to write a literal `@`:

```text
Contact support@@example.com
```

The output contains `support@example.com`.

Template comments are omitted from the output:

```text
@* This comment is not rendered. *@
```

Inside Rust code, use Rust comments:

```text
@{
    // A Rust line comment.
    let count = 3; /* A Rust block comment. */
}
@(count)
```

HTML comments such as `<!-- comment -->` are literal content, not template
comments.

## Imports

Use `@use` to import Rust types or functions:

```text
@use crate::view_models::IndexModel;
@model IndexModel;
<p>@(model.name)</p>
```

Place declarations at the top level and end them with a semicolon. Paths must
be valid and accessible from the generated Rust modules.

## Typed models

Declare the template's model type at the top level:

```text
@model crate::view_models::IndexModel;
<p>@(model.name) is @(model.age).</p>
```

The type above is the model from [Get started](./get_started.md).
It looks up `Arc<IndexModel>` under context key `"model"`.
Use an unquoted identifier and a colon to select a different key:

```text
@model layout: crate::LayoutModel;
<title>@(model.title)</title>
@render
```

This looks up `Arc<LayoutModel>` under key `"layout"`; the template variable
is still named `model`. With no declaration, the template has no typed model
and performs no automatic model lookup.

The declaration emits no output and applies to this template's file and kind.
Types can use `@use` imports. Only one declaration is allowed, and `@model`
cannot appear inside code, sections, or nested content blocks. A declaration
ends at a semicolon, a newline after a complete Rust type, or the end of the file.
To interpolate the model itself, use `@(model)` rather than the reserved
`@model` directive. The model must implement a supported formatting trait.

Both the named API and a generated view's rendering methods retrieve
`Arc<Model>` under the declared key.
The template variable `model` holds that cloned `Arc<Model>`; field access
automatically dereferences it.
Models must be `Send + Sync + 'static`.
Missing keys or incorrectly typed values produce explicit rendering errors.

Use `sbolt::context!(model_value)` as shorthand for
`sbolt::context!(model: std::sync::Arc::new(model_value))`.
Explicit keys and multiple entries store values as supplied. For an existing
`Arc<Model>`, use `sbolt::context!(model: model_arc)` to avoid wrapping it again.

Struct fields retain their declared Rust types. For a `String` field,
`name: "sbolt".into()` is concise, but Rust does not implicitly convert a string
literal during struct construction.

Layouts independently retrieve their models from their own declared keys;
they never inherit the caller's model or fall back to another key.
Custom views likewise use their own declarations and the context passed to
`@view`. See
[typed rendering APIs](../dev/template-compilation.md#optional-typed-models).

## Specialized interpolation

In HTML and text templates, interpolation selects a renderer using Rust's type
system, without a compiler option:

1. `sbolt::types::WriteTo`, when implemented.
2. Streaming `std::fmt::Display` formatting.

Types implementing only `ToString` are no longer supported directly. Implement
`Display` or `WriteTo` when migrating, or explicitly interpolate
`@(value.to_string())` if an allocating conversion is intentional.

Built-in `WriteTo` implementations append strings directly, write booleans as
`true` or `false`, and format integer primitives with an `itoa` stack buffer.
References forward to the underlying implementation. Floats and other `Display`
values use streaming formatting. Custom formatting implementations may still
allocate internally.

```text
@(1.25)
@(true)
```

Custom types can implement `WriteTo` to override their `Display` formatting:

```rust
use sbolt::types::{WriteTo, Writer};

pub struct Status(pub bool);

impl WriteTo for Status {
    fn write_to(&self, writer: &mut impl Writer) {
        writer.write(if self.0 { "up" } else { "down" });
    }
}
```

`sbolt::types::display(value)` explicitly forces `Display` formatting.
`DisplayOption` and `DisplayOptionRef` format a present value or emit nothing for
`None`; an ordinary `Option<T>` does not automatically become renderable:

```text
@{
    let count: Option<i32> = None;
}
@(sbolt::types::DisplayOption(count))
```

### Output safety

HTML expressions such as `@(value)` automatically escape `&`, `<`, `>`, `"`, and
`'`, without a compiler option. Literal
template markup is unchanged. For example, a value of `<em>Hello</em>` in
`<p>@(value)</p>` is displayed as text, not interpreted as an element.
JSON expressions automatically serialize values; text expressions retain ordinary
formatting. HTML escaping preserves the specialized interpolation priority above
and streams output.

Use `sbolt::utils::raw(...)` only for explicitly trusted HTML in autoescaped templates:

```text
<p>@(message)</p>
@(sbolt::utils::raw("<strong>Trusted markup</strong>"))
```

The raw wrapper bypasses escaping; it does not sanitize HTML. It is intended for
HTML expressions, not JSON/text expressions. Raw wrappers can be borrowed and
reused, including values retrieved from context. Child views and rendered sections
are already-rendered output and are inserted directly, without escaping their
markup a second time. Expressions inside HTML child views and sections are escaped.

This changes the previous raw-by-default HTML behavior. When migrating templates,
remove manual HTML escaping from ordinary expressions to avoid double escaping.
Replace intentionally interpolated trusted markup with `sbolt::utils::raw(...)`, or
prefer a child view. `sbolt::utils::escape_html(value)` remains available for explicit
encoding outside normal HTML interpolation.

#### Insertion contexts

HTML escaping is appropriate for HTML text and quoted attribute values. Always
quote dynamic attribute values. It does not validate URL schemes or make values
safe inside scripts, styles, or event-handler attributes. Validate URLs and use
destination-specific serialization or encoding. Templates execute trusted Rust;
neither escaping nor a fresh context makes a template a sandbox.

#### JSON templates

In `.rsjson` templates, each expression must implement `serde::Serialize` and
represents a complete JSON value:

```text
@{
    let name = "Tom & Jerry";
    let items = [1, 2];
}
{"name": @(name), "items": @(items)}
```

Strings are quoted and escaped, arrays and objects retain their structure, and
`Option::None` becomes `null`. Serialization streams into the output buffer
without an intermediate JSON string. Serialization failures propagate as rendering
errors. Closures containing JSON interpolation must return a compatible `Result`;
use `try_for_each` rather than `for_each`.

Model bindings can be interpolated directly with `@(model)` when the declared type
implements `Serialize`; their `Arc` wrapper is handled automatically.

When migrating, remove quotes around interpolated values and remove explicit
`json(...)` calls. For example, replace `"name": "@(name)"` with
`"name": @(name)`. An interpolated JSON string is still a string, so serializing
it explicitly first would double-encode it. Use ordinary `Option<T>` values
instead of the HTML/text formatting wrappers `DisplayOption` and `DisplayOptionRef`.

For pre-serialized JSON, use `@(sbolt::utils::raw_json(pre_serialized)?)`, where
`pre_serialized` is a `&str`. This validates one complete JSON value and inserts it
without extra quotes. It is distinct from the trusted-HTML `raw` wrapper.

Literal content, rendered sections, and child-view output are not serialized again.
Individual values are serialized safely, but the complete dynamic JSON document
is not validated.

#### JSON data in HTML

Use `sbolt::utils::json(&value)` without enabling a Cargo feature. It accepts
`serde::Serialize` values and returns a rendering error if serialization fails.
It serializes JSON and replaces `<`, `>`, `&`, `'`, U+2028, and U+2029 with JSON
Unicode escapes, preserving the original values when parsed.

In an HTML template:

```text
@model String;
<script id="page-data" type="application/json">@(sbolt::utils::raw(sbolt::utils::json(model.as_ref())?))</script>
<script>
    const data = JSON.parse(document.getElementById("page-data").textContent);
</script>
```

Here, the JSON helper handles script-data escaping; `raw` prevents HTML entities
from corrupting the JSON. Do not add quotes around the serialized value.
For a quoted HTML attribute, let HTML autoescaping escape the serialized result
instead of wrapping it in `raw`. Do not interpolate it inside an existing
JavaScript string or an event handler.

JSON templates apply the same JSON escaping automatically; they do not need this
helper. Embedding a JSON document in a quoted HTML attribute still requires HTML
escaping.

## Named context values

A template without `@model` can read values directly:

```text
@{
    let name = sbolt::types::DisplayOptionRef(context.get_data::<String>("name"));
}
<p>@(name)</p>
```

For this example, supply `sbolt::context!(name: String::from("sbolt"))`.
`get_data::<T>` requires the exact stored type; a stored string literal is an
`&'static str`, not a `String`. This example intentionally renders an empty
value when the key is missing or has a different type.

The [context guide](./get_started.md#contexts) covers ownership and write-once
data keys. If a context borrow conflicts with nested mutable rendering, clone
the needed data or use a typed model, whose `Arc` is cloned by the renderer.

## Custom views

Use `@view(name, context)` to render another template at the current position.
For example, create `src/views/card.rshtml`:

```text
@{
    let title = sbolt::types::DisplayOptionRef(context.get_data::<String>("title"));
}
<aside>@(title)</aside>
```

Insert it from another template:

```text
<main>
    @view("views/card", sbolt::context!(title: String::from("Welcome")))
</main>
```

- The first argument is a Rust expression producing a registered view name as `&str`.
  Use the same registered path as the generated `render` API, without a template
  extension. Include the source directory component when it is part of the
  registered name, as in `"views/card"`.
- The second argument is a context expression. `sbolt::context!(...)` creates a
  fresh context containing named values. Values must satisfy
  `Send + Sync + 'static`; use owned values for data borrowed from local variables.
  If `context` is imported, the shorter `context!(...)` spelling also works.
  Existing mutable contexts and mutable references are also accepted, including
  `@view("views/card", context)` and
  `@view("views/card", &mut child_context)`. References are reborrowed, so
  the caller can continue using them afterward.
- The optional third argument is a Rust expression producing
  `sbolt::types::template::Kind`. Omit it to inherit the caller's kind, or provide
  a kind directly (not `Some(...)` or `None`) to override it.
- Two or three arguments are accepted; a trailing comma is allowed.
- Arguments are evaluated exactly once, from left to right: name, context,
  then the optional kind override.

Unlike `@render(name)`, which renders a section from the context, `@view(...)`
resolves and renders a separate template. It also works inside sections and Rust
code blocks, including loops.

To select a different kind explicitly:

```text
@view("views/data", sbolt::context!(), sbolt::types::template::Kind::KJSON)
```

This selects the JSON child even in an HTML parent. Output is still inserted
directly: choosing JSON does not make it safe to embed in an HTML attribute or
script element. Apply escaping or serialization appropriate to the insertion
location.

### Choosing a context

Prefer a fresh, explicit `sbolt::context!(...)` for each child, as in the
examples above. Pass only the values the child needs; parent data and sections
are not inherited, and child section changes stay in that separate context.

Share the parent's context only when shared data or section mutation is
intentional:

```text
@view("views/card", context)
```

In this case, the child and its layout can modify the parent's data and sections,
including the default section. Reborrowing does not undo those changes.
Both forms still require an explicit context argument; there is no implicit
parent-context fallback.

### Rendering behavior

- **Lookup:** names are resolved from the generated view registry, not relative
  to the caller. There is no layout-style parent-directory search.
- **Kind:** the child uses the caller's HTML, JSON, or text kind unless the
  third argument overrides it. There is no fallback to another kind.
- **Context:** only the supplied context is used. A fresh `context!(...)` does
  not inherit parent data or sections, and sections created by the child remain
  in that child context. Explicitly passing the parent's `context` instead shares
  its data and sections, including any section changes made by the child or its
  layout.
- **Layouts:** the child template's declared layout is applied using the selected
  child kind, just as when rendering the child directly. An override does not
  change the parent's kind or layout.
- **Output:** the rendered string is inserted directly, without additional
  escaping. In JSON templates, place the directive where the child's JSON output
  forms a valid value; the combined dynamic output is not validated.
- **Errors:** missing views, missing matching kinds, and child rendering errors
  propagate to the caller as rendering errors.

Only `@view(...)` is supported; `@Custom::View(...)` is not an alias.

[Get started](./get_started.md) | [Keywords](./keywords.md) | [Layouts](./layout.md)