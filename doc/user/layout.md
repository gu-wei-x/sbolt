# Layouts and sections

A layout wraps a page with common content such as navigation, a header, or a
footer. Named sections let the page supply content for specific positions.
Use [custom views](./syntax.md#custom-views) for reusable fragments inserted
inline instead of wrapping the whole page.

## A complete layout pair

```text
src/views/
|-- layout.rshtml
`-- pages/
    `-- index.rshtml
```

`src/views/layout.rshtml`:

```text
<html>
    <head>@render(title)</head>
    <body>
        <nav>My application</nav>
        <main>@render</main>
        @render(scripts, false)
    </body>
</html>
```

`src/views/pages/index.rshtml`:

```text
@layout layout;
@section title {
    <title>Home</title>
}
<h1>Home</h1>
<p>This is the page body.</p>
```

Render `"views/pages/index"` using the generated renderer. sbolt:

1. Renders the page and stores its named sections in the context.
2. Resolves the layout using the page's kind.
3. Stores the page body as the default section.
4. Renders the layout with the same context.

The layout inserts the title and page body. The missing optional `scripts`
section emits nothing. Rendering `"views/layout"` directly without supplying
its required sections returns an error.

## Layout lookup

Layout paths are unquoted and omit the template extension.

| Declaration | Lookup for a page in `views/pages/` |
| --- | --- |
| `@layout layout;` | Try `views/pages/layout`, then `views/layout`, then `layout` |
| `@layout ~/layout;` | Try only `views/pages/layout` |
| `@layout /views/layout;` | Try only `views/layout` |

The selected layout must have the same `Kind` as the page. An HTML page cannot
silently use a JSON layout when its HTML layout is missing.

Layouts may declare their own layout, creating a chain of wrappers. Each step
replaces the default section with the preceding step's output. Do not create
cyclic layout chains.

## Sections

Define a section with an unquoted identifier and a content block:

```text
@section scripts {
    <script src="/first.js"></script>
}
@section scripts {
    <script src="/second.js"></script>
}
```

Both fragments are appended under `"scripts"` and later rendered in that order.
Sections can contain expressions, Rust code blocks, and `@view` calls. Their
content is rendered when the section is defined, not deferred until `@render`.
Defining a section does not emit it at its original position.

### Required and optional sections

```text
@render(title)
@render(title, true)
@render(scripts, false)
```

- `@render(title)` and `@render(title, true)` return `RuntimeError::NotFound`
  when `"title"` is absent.
- `@render(scripts, false)` emits nothing when `"scripts"` is absent.
- Present sections render all stored fragments in order.

The name is an identifier, not a quoted string. The optional second argument
must be a boolean literal.

### The default section

Use `@render` or `@render()` to insert the body supplied by the page. Both forms
require a default section; neither has an optional boolean form.

The default section is replaced at each layout step, while named sections
append. Section state remains in the context after rendering. Prefer a fresh
context for each page or HTTP request to avoid carrying earlier sections over.

## Independent layout models

A page and its layout retrieve their models independently. Use separate keys
when they have different model types:

```rust
pub struct PageModel {
    pub heading: String,
}

pub struct LayoutModel {
    pub title: String,
}
```

For this example, place both types in `crate::view_models`. The page declares:

```text
@model crate::view_models::PageModel;
@layout layout;
<h1>@(model.heading)</h1>
```

The layout declares:

```text
@model layout: crate::view_models::LayoutModel;
<html>
    <head><title>@(model.title)</title></head>
    <body>@render</body>
</html>
```

Supply both models:

```rust
let mut context = sbolt::context!(
    model: std::sync::Arc::new(view_models::PageModel {
        heading: "Home".into(),
    }),
    layout: std::sync::Arc::new(view_models::LayoutModel {
        title: "My application".into(),
    })
);
```

Only the single-expression context shorthand inserts an `Arc` automatically.
Explicit named entries above must contain the required `Arc<Model>` values.
The layout's variable is still named `model`, even though its context key is
`"layout"`. Missing or incorrectly typed models produce `RuntimeError::InvalidInput`;
there is no fallback to the page's model.

## Errors and output safety

Missing layouts and required sections are rendering errors. They propagate
through the page's render result rather than selecting a different template
kind. Layouts and sections insert rendered output without additional escaping.
HTML expressions inside them are escaped automatically; expressions inside JSON
templates are automatically serialized. See [output safety](./syntax.md#output-safety)
for limitations.

[Get started](./get_started.md) | [Syntax](./syntax.md) | [Keywords](./keywords.md)