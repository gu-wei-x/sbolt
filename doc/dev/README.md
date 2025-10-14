# Dev

Prerequisites: 
* OS --- Windows/Linux/macOS
* IDE --- [Visual Studio Code](https://code.visualstudio.com)

## 1. Install `rust` toolchain

Use `rustup` to install the latest stable of `rust`. Install `rustup` by following the instructions on [rustup](https://rustup.rs/).
Once `rustup` is installed, ensure the latest toolchain is installed by running the command:
```sh
rustup default stable
```

## 2. Build and run test

### lib: `cd` to directory [rust](../../rust)
* build 
```shell
cargo build
```

* test
```shell
cargo test
```

* benchmark: `cd` to directory [rust/benchmark](../../rust/benchmark)
```shell
cargo bench
```

### examples: `cd` to directory [rust/examples](../../rust/examples)
* build 
```shell
cargo build
```

* run examples: rocket-example for example, name is from each Cargo.toml file in directory [examples](../../rust/examples)
```
cargo run -p rocket-example
```

```
% cargo run -p rocket-example
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `../../target/example/debug/rocket-example`
```

### clean build artifacts: `cd` to directory [rust](../../rust) and run:

```shell
cargo clean
```

## 3. Workflow

* run [rust/benchmark](../../rust/benchmark) and [code coverage](./coverage.md)
* add features and tests
* run [rust/benchmark](../../rust/benchmark) and [code coverage](./coverage.md)
* don't downgrade the perf and code coverage

## 4. Useful commands

* verbose: `cargo build -vv`
* show test output: `cargo test [test_name|*] -- --nocapture`

## 5. links
* [Rust Documentation](https://doc.rust-lang.org/stable)
