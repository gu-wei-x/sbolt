# CLI examples

Two command-line examples demonstrate rendering with the
[Sbolt template engine](../../../../README.md).

## Project structure

The examples belong to the [CLI workspace](../../../../rust/examples/cli/Cargo.toml):

- [basic](../../../../rust/examples/cli/basic) (package `cli-basic`) calls the renderer
  exposed by the shared crate. It needs no template build script of its own.
- [standalone](../../../../rust/examples/cli/standalone) (package `cli-standalone`)
  compiles the shared templates in its own build script and renders them through
  `cli_standalone_views`.
- [shared](../../../../rust/examples/cli/shared) (package `example_views`) contains
  models, templates, and a shared renderer. Both applications depend on it.
  The standalone application uses its models, but the dependency also builds
  the shared renderer.

```text
rust/examples/cli/
|-- basic/
|   |-- Cargo.toml
|   `-- src/
|-- standalone/
|   |-- Cargo.toml
|   |-- build.rs
|   `-- src/
`-- shared/
    |-- Cargo.toml
    |-- build.rs
    |-- src/
    `-- views/
```

## Run

From the repository root:

```powershell
cd rust\examples\cli
cargo run -p cli-basic
cargo run -p cli-standalone
```

Both applications print an HTML page containing
`<div>Welcome! - from sbolt(1)</div>`.

## Tests

From the same directory:

```powershell
cargo test --workspace
```

Tests cover model rendering, HTML escaping, JSON serialization, and standalone
template compilation. Build output goes to `target/examples/cli` at the repository root.

See the [getting-started guide](../../user/get_started.md) for template compilation
and rendering details.
