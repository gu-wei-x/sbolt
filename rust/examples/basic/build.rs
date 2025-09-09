// `cargo build -vv` to show output.
use disguise;
use std::env;

fn main() {
    _ = disguise::process_views("src/views", &format!("{}_views", env!("CARGO_PKG_NAME")));
}
