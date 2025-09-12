// Import the generated view modules.
disguise::include_view_templates!();

fn main() {
    // create a context and set some data.
    let mut context = disguise::context! {
        strvalue => || "Hello, world!".to_string(),
        intvalue => || 123
    };
    basic_views::render("comp/index", &mut context);
    println!("{}", context);
}
