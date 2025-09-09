# disguise

view template engine in rust

## 1. create views directory containing view templates(*.rshtml)
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

## 2. create build.rs, call `process_views` to pre-process views.
```rust

use disguise;
use std::env;

fn main() {
    _ = disguise::process_views("src/views", &format!("{}_views", env!("CARGO_PKG_NAME")));
}

```

## 3. main.rs: include generated-file map and call logic to render view ----TODO: add context/model for the view.

```rust
include!(env!("VIEW_FILES"));

fn main() {
    let view = /*cratename_views*/basic_views::get_view("test" /*"comp/index*/);
    if let Some(view) = view {
        let output = view.render();
        println!("{}", output);
    }
}
```

## 4. run

```sh
cargo run
```

