// Import the generated view modules.
disguise::include_view_templates!();

fn main() {
    // create a context and set some data.
    let context = disguise::context! {
        name: "Disguise".to_string(),
        age: 1,
        msg: "Hello world!".to_string()
    };
    let mut output = String::new();
    basic_views::render("views/comp/index", context, &mut output);
    println!("{}", output);
}
