// Import the generated view modules.
disguise::include_views!();

fn main() {
    // create a context and set some data.
    let context = disguise::context! {
        name: "Disguise".to_string(),
        age: 1,
        msg: "Hello world!".to_string()
    };
    let output = basic_views::render("views/comp/home", context).unwrap_or_else(|e| {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    });
    println!("{output}");
}
