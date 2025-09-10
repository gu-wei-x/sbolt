# disguise

view template engine in rust. disguise pre-processes templates from a directory and compiles templates into crate bits.

## Following is the steps to use this view template engine. See: [examples/basic](./rust/examples/basic/)

### 1. add disguise crate and add disguise to `[build-dependencies]`
```shell
cargo add disguise
```

### 2. create views directory containing view templates(*.rshtml)
```
$:.
│   build.rs
│   Cargo.toml
│
└───src
    │   main.rs
    │
    └───views
        │   test.rshtml
        │
        └───comp
                index.rshtml


```
src/views/comp/index.rshtml
```html
<html>
    <head>
        <title>Index</title>
    </head>
    <body>
        <div>Hello Index!</div> 
    </body>
</html>
```

### 3. build.rs: call `process_views` to pre-process views in build script.

```rust
use disguise;
use std::env;

fn main() {
    _ = disguise::process_views("src/views", &format!("{}_views", env!("CARGO_PKG_NAME")));
}
```

### 4. main.rs: include generated views and call logic to render view.

```rust
use disguise::types::{ViewContext, Writer};

// include the generated views.
disguise::include_view_templates!();

fn main() {
    let mut output = String::new();
    let mut context: ViewContext<'_, dyn Writer> = ViewContext::new(&mut output);
    basic_views::render("comp/index", &mut context);
    println!("{}", output);
}
```

### 5. run

```sh
$>cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target\debug\basic.exe`
<html>
    <head>
        <title>Index</title>
    </head>
    <body>
        <div>Hello Index!</div>
    </body>
</html>
```

