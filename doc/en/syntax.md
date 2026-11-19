# Syntax

sbolt template files contain markup syntax for embedding `rust` code into content. The syntax consists of markup, `rust` and text. Template files usually use `.rshtml` for html, `.rsjson` for json and `.rstxt` for plain text. The extention could be overrided by compile options. sbolt uses `@` symbol to ransition between `rust` code and content. 

## Redering HTML

The default template is for rendering HTML content. There is no difference for rendering plain HTML from template with markup syntax. A HTML template usually uses `.rshtml` as the extention.

## Basic Syntax

sbolt template supports rust and uses the `@` symbol to transition from HTML content to `rust` code. sbolt generate `rust` expressions at build stage to be envaluated later at runtime. When an `@` symbol is followed by sbolt [keywords](./keywolds.md), it transitons into specific markup. Otherwise, it transitions into HTML text. To escap an `@` symbol in markup, use a second `@` symbol: 

- **escape `@`:** use double `@`.
    ```
    Hello, @@world!
    ```

- **code:**: use single `@` to render code in content.
    ```
    Hello, @Username!
    ```

## Code block

Code blocks start with `@`,  start with `@` enclosed by `{}` | `()`. Unlike expressions, `rust` code inside code blocks isn't rendered. Code blocks and expressions in a tempalte share the same scope and are defined in order:

```rust
@{
    let msg = "hello world!";
}
<p>@msg</p>
@{
    let msg = "hello rust!";
}
<p>@msg</p>
<p>@(msg)</p>
```

The code renders the following HTML content:
```html
<p>hello world!</p>
<p>hello rust!</p>
```

### tansition to HTML
The default language in a code block is `rust`, but it could be transition back to HTML content:

- **use `@{}`** to transition to block of HTML content. 
```
@{
    let msg = `hello world!`;
    @{<p>@msg</p>}
}
```

- **use `@`** to transition to small block of HTML content. 
```
@{
    let msg = `hello world!`;
    @msg
}
```

- **use `@()`** to transition to small block of HTML content. 
```
@{
    let msg = `hello world!`;
    @(@msg)
}
```

## Comments

Add comments that will not appear in the output:

- **use** `@**@` to add comments in HTML content
```
@** this is comment **@
```

- **use** rust comment syntax to add comments in code
```
@{
    // this is comment
    /*this is comment*/
}
```

## HTML content block

HTML content blocks start without `@` symbol or start with `@` followed by content keywords like `section`
```
this is content
@section test {
    this is section content.
}
```
Refer to the [layout](./layout.md) section for advanced usage.

- **use `@{}`** to transition to `rust`. 
```
this is content
@{
    let msg = `hello world!`;
}
this is content
```

- **use `@`** to transition to small block of `rust`. 
```
this is content, render @Username here.
```

- **use `@()`** to transition to small block of `rust`. 
```
this is content, render @(Username) here.
```

## Custom views

Use `@view(name, context)` to render another template at the current position.
This is useful for reusable content such as cards, navigation, and headers.

```text
@view("views/shared/card", sbolt::context!(title: "Welcome".to_string(), count: 3usize))
```

- The first argument is a Rust expression producing a view name as `&str`.
  Use the same registered path as the generated `render` API, without a template
  extension. Include the source directory component when it is part of the
  registered name, as in `"views/shared/card"`.
- The second argument is a context expression. `sbolt::context!(...)` creates a
  fresh context containing named values. Values must satisfy
  `Send + Sync + 'static`; use owned values for data borrowed from local variables.
  If `context` is imported, the shorter `context!(...)` spelling also works.
  Existing mutable contexts and mutable references are also accepted, including
  `@view("views/shared/card", context)` and
  `@view("views/shared/card", &mut child_context)`. References are reborrowed, so
  the caller can continue using them afterward.
- The optional third argument is a Rust expression producing
  `sbolt::types::template::Kind`. Omit it to inherit the caller's kind, or provide
  a kind directly (not `Some(...)` or `None`) to override it.
- Two or three arguments are accepted; a trailing comma is allowed.
- Arguments are evaluated exactly once, from left to right: name, context,
  then the optional kind override.

For example:

```text
<main>
    <h1>Dashboard</h1>
    @view("views/shared/card", sbolt::context!(title: "Welcome".to_string(), count: 3usize))
</main>
```

Unlike `@render(name)`, which renders a section from the context, `@view(...)`
resolves and renders a separate template. It also works inside sections and Rust
code blocks, including loops.

To select a different kind explicitly:

```text
@view("views/shared/data", sbolt::context!(), sbolt::types::template::Kind::KJSON)
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
@view("views/shared/card", context)
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

`@Custom::View(...)` was an earlier proposal, not a supported alias.

---
### [Next: layout](./layout.md)