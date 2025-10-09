disguise::include_views!();

pub fn render_template_with_large_size_of_content() {
    _ = bench_views::render("views/large", &mut disguise::context!());
}

pub fn render_template_with_layout() {
    _ = bench_views::render("views/testlayout", &mut disguise::context!());
}

pub fn render_template_without_layout() {
    let mut context = disguise::context! {
        name: "Disguise".to_string(),
        age: 1,
        msg: "Hello world!".to_string()
    };
    _ = bench_views::render("views/test", &mut context);
}
