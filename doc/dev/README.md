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

### lib: `cd` to directory [rust/core](../../rust/core)
* build 
```shell
cargo build
```

* test
```shell
cargo test
```

Core build output goes to `target/core` at the repository root.

* benchmark: `cd` to directory [rust/core/lib_benchmark](../../rust/core/lib_benchmark)
```shell
cargo bench
```

### CLI examples: `cd` to directory [rust/examples/cli](../../rust/examples/cli)
* build 
```shell
cargo build
```

* run either CLI example:
```shell
cargo run -p cli-basic
cargo run -p cli-standalone
```

### Web examples: `cd` to directory [rust/examples/web](../../rust/examples/web)

Roster's Actix Web, Axum, and Rocket applications share the web workspace.
Build and test them together:

```shell
cargo build --workspace
cargo test --workspace
cargo run -p roster-rocket
```

Use `roster-actix` or `roster-axum` for the other frameworks. All three default
to `http://127.0.0.1:3000/`, so run only one at a time.

CLI and web builds go to `target/examples/cli` and `target/examples/web`,
respectively, at the repository root. Run Cargo from the corresponding workspace
directory so it loads that workspace's Cargo configuration.

### Bundle tool

The [app-bundle tool](../../rust/tools/app-bundle) is a standalone package,
separate from the [core workspace](../../rust/core/Cargo.toml).
From `rust/examples/web`, use the
`cargo bundle` alias to package a web example and its assets:

```shell
cargo bundle --package roster-axum --profile release
```

The tool has no Cargo configuration of its own. The web alias runs from
`rust/examples/web`, so it uses the web configuration and builds the tool into
`target/examples/web`.

See the [Roster deployment guide](../guides/examples/roster/README.md#deployment).

### clean core build artifacts: `cd` to directory [rust/core](../../rust/core) and run:

```shell
cargo clean
```

## 3. Workflow

* run [rust/core/lib_benchmark](../../rust/core/lib_benchmark) and [code coverage](./coverage.md)
* add features and tests
* run [rust/core/lib_benchmark](../../rust/core/lib_benchmark) and [code coverage](./coverage.md)
* don't downgrade the perf and code coverage

## 4. Useful commands

* verbose: `cargo build -vv`
* show test output: `cargo test [test_name|*] -- --nocapture`

## 5. links
* [Development plan](./plan.md)
* [Benchmarks](./benchmark.md)
* [Template compilation](./template-compilation.md)
* [Rust Documentation](https://doc.rust-lang.org/stable)
