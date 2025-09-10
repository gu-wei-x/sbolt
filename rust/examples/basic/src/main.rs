use disguise::types::{ViewContext, Writer};

// include the generated views.
disguise::include_view_templates!();

fn main() {
    let mut output = String::new();
    let mut context: ViewContext<'_, dyn Writer> = ViewContext::new(&mut output);
    basic_views::render("comp/index", &mut context);
    println!("{}", output);
}
