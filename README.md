# sbolt
[<img alt="github" src="https://img.shields.io/badge/github-guweix/sbolt-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/gu-wei-x/sbolt)
[<img alt="crates.io" src="https://img.shields.io/crates/v/sbolt.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/sbolt)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-sbolt-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/sbolt)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/gu-wei-x/sbolt/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/gu-wei-x/sbolt/actions?query=branch%3Amain)

sbolt is a view template engine in rust. sbolt pre-processes templates from directories and compiles them into crate bits. 

## sbolt has benefits compared to other templates:
1. Catch syntactical and logical errors in templates ahead of time.
2. High scalability as templates go with crate bits.
3. Better performance for runtime and startup as templates are compiled into bits, see [benchmark](./rust/benchmark).

The template syntax is inspired by [aspnet Razor](https://dotnet.microsoft.com/en-us/apps/aspnet) syntax which uses `@` symbol to transition between `CONTENT` and `rust` code. A sample template looks llike bellow: `views/welcome.rshtml`
```
@{
    let name = match context.get_data::<String>("name") {
        Some(str) => str,
        None => "",
    };
}
<html>
    <head>
        <title>Welcome</title>
    </head>
    <body>
        <div>Welcome @name!</div>
    </body>
</html>
```

### Getting started
The goal of sbolt is to have your templates and static files accessible to your rust code as a module. And the module will be generated into code living outside of src directory. There are 3 steps to use the crate:

1. use sbolt to preprocess templates at build stage.
>> * add the crate to deps: `Cargo.toml`
```shell
cargo add sbolt
cargo add sbolt --build
```
>> * create a build script to process templates: `build.rs`
```rust
fn main() {
    let option = sbolt::codegen::CompilerOptions::default()
        .with_optimize(true)
        .with_source_dir("src/views")
        .with_mod_name("example_views");
    let compiler = sbolt::codegen::Compiler::new(option);
    compiler.compile();
}
```
2. include the generated module from build stage and use the logic from generated module.
```rust
sbolt::include_views!();
fn main() {
    let mut context = sbolt::context! {
        name: "sbolt".to_string()
    };
    let output = example_views::render("views/welcome", &mut context).unwrap_or_else(|e| {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    });
    println!("{output}");
}
```
3. build and run your crate.


## Documentation
  * [Dev](./doc/dev/README.md)
  * [User](./doc/en/get_started.md)

## License

this crate is licensed under [LICENSE](./LICENSE)

