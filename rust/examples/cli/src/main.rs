// Import the generated view modules.
disguise::include_views!();

fn main() {
    // create a context and set some data.
    let mut context = disguise::context! {
        name: "Disguise".to_string(),
        age: 1,
        msg: "Welcome!".to_string()
    };
    let output = cli_views::render("views/sub/index", &mut context).unwrap_or_else(|e| {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    });
    println!("{output}");
}
