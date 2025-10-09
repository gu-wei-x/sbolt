# crate disguise

disguise is a view template engine in rust. disguise pre-processes templates from directories and compiles them into crate bits. 

## disguise has benefits compared to other templates:
1. Catch syntactical and logical errors in templates ahead of time.
2. High scalability as templates go with crate bits.
3. Better performance for runtime and startup as templates are compiled into bits.

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
The goal of disguise is to have your templates and static files accessible to your rust code as a module. And the module will be generated into code living outside of src directory. There are 3 steps to use the crate:

1. use disguise to preprocess templates at build stage.
>> * add the crate to deps: `Cargo.toml`
```shell
cargo add disguise
cargo add disguise --build
```
>> * create a build script to process templates: `build.rs`
```rust
fn main() {
    let option = disguise::codegen::CompilerOptions::default()
        .with_optimize(true)
        .with_source_dir("src/views")
        .with_mod_name("example_views");
    let compiler = disguise::codegen::Compiler::new(option);
    compiler.compile();
}
```
2. include the generated module from build stage and use the logic from generated module.
```rust
disguise::include_views!();
fn main() {
    let mut context = disguise::context! {
        name: "disguise".to_string()
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
  * [Userdoc](./doc/en/get_started.md)

## License

this crate is licensed under [LICENSE](./LICENSE)

