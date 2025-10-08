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
```rust
@{
    let name = match self.get_data::<String>("name") {
        Some(str) => str,
        None => "",
    };
    let age = disguise::types::DisplayOptionRef(self.get_data::<i32>("age"));
    let msg = disguise::types::DisplayOptionRef(self.get_data::<String>("msg"));
}
<html>
    <head>
        <title>Index</title>
    </head>
    <body>
        <div>@msg - from @name(@age)</div>
    </body>
</html>
```

### 3. build.rs:

```rust
// `cargo build -vv` to show output.
fn main() {
    let mod_name = format!("{}_views", std::env!("CARGO_PKG_NAME"));
    let option = disguise::codegen::CompilerOptions::default()
        .with_optimize(true)
        .with_source_dir("src/views")
        .with_mod_name(&mod_name);
    let compiler = disguise::codegen::Compiler::new(option);
    compiler.compile();
}
```

### 4. main.rs: include generated views and render view.

```rust
// Import the generated view modules.
disguise::include_views!();

fn main() {
    // create a context and set some data.
    let mut context = disguise::context! {
        name: "Disguise".to_string(),
        age: 1,
        msg: "Hello world!".to_string()
    };
    let output = basic_views::render("views/comp/index", &mut context).unwrap_or_else(|e| {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    });
    println!("{output}");
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
        <div>Hello world! - from Disguise(1)</div>
    </body>
</html>
```

