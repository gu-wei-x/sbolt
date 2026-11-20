fn main() {
    let mut context = sbolt::context!(example_views::models::IndexModel {
        name: "sbolt".into(),
        age: 1,
        msg: "Welcome!".into(),
    });
    let (output, _) =
        example_views::render("views/sub/index", None, &mut context).unwrap_or_else(|e| {
            eprintln!("Error: {e:?}");
            std::process::exit(1);
        });
    println!("{output}");
}

#[cfg(test)]
mod tests {
    use sbolt::types::{error::RuntimeError, template::Kind};
    use std::sync::Arc;

    #[test]
    fn index_renders_model_fields() {
        let mut context = sbolt::context!(example_views::models::IndexModel {
            name: "sbolt".into(),
            age: 1,
            msg: "Welcome!".into(),
        });
        let (output, _) = example_views::render("views/sub/index", None, &mut context).unwrap();
        assert!(output.contains("<div>Welcome! - from sbolt(1)</div>"));
    }

    #[test]
    fn data_driven_views_require_the_declared_model() {
        for (path, kind) in [
            ("views/sub/index", Kind::KHTML),
            ("views/welcome", Kind::KHTML),
            ("views/welcome", Kind::KJSON),
            ("views/welcome", Kind::KTEXT),
        ] {
            for mut context in [
                sbolt::context!(),
                sbolt::context!(model: Arc::new(String::from("wrong type"))),
                sbolt::context!(name: String::from("legacy data")),
            ] {
                assert!(matches!(
                    example_views::render(path, Some(kind), &mut context),
                    Err(RuntimeError::InvalidInput(message, _)) if message.contains("model")
                ));
            }
        }
    }
}
