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

---
### [Next: layout](./layout.md)