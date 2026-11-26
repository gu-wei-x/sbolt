# Examples

Runnable examples for the [Sbolt template engine](../../../README.md).
All examples use the local library. The
[CLI workspace](../../../rust/examples/cli/Cargo.toml) and
[web workspace](../../../rust/examples/web/Cargo.toml) are independent.

## CLI

The [CLI examples](cli/README.md) demonstrate rendering through a shared
template crate and compiling templates directly in an application.

## Roster

The [Roster example](roster/README.md) demonstrates a company and employee
management application with Axum, Actix Web, or Rocket, shared HTML templates,
and SQLite storage. The guide includes an animated UI tour.

The repository also provides an
[app-bundle tool](../../../rust/tools/app-bundle) for packaging executables and
their assets. The web workspace exposes it through `cargo bundle`; run this
command from `rust/examples/web`. See the [deployment instructions](roster/README.md#deployment).

## Run

Install stable Rust with edition 2024 support. From the repository root:

```powershell
cd rust\examples\cli
cargo run -p cli-basic
cargo run -p cli-standalone
```

For a web application, start from the repository root and choose one of
`roster-axum`, `roster-actix`, or `roster-rocket`:

```powershell
cd rust\examples\web
cargo run -p roster-axum
```

Open <http://127.0.0.1:3000/>. Run only one web application at a time with the
default configuration.

Build and test all examples in either workspace from its directory:

```powershell
cargo build --workspace
cargo test --workspace
```

Run these checks in both workspaces to cover all examples. CLI builds go to
`target/examples/cli`; web builds go to `target/examples/web` at the repository
root. Run Cargo from the workspace directory (or a child directory) so it loads
the corresponding `.cargo/config.toml`.
