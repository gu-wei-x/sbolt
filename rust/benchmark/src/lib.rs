sbolt::include_views!();

pub fn render_template_with_large_size_of_content() {
    _ = bench_views::render("views/large", &mut sbolt::context!());
}

pub fn render_template_with_layout() {
    _ = bench_views::render("views/testlayout", &mut sbolt::context!());
}

pub fn render_template_without_layout() {
    let mut context = sbolt::context! {
        name: "sbolt".to_string(),
        age: 1,
        msg: "Hello world!".to_string()
    };
    _ = bench_views::render("views/test", &mut context);
}
