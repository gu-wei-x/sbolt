// This example uses example_views only for model types and renders through
// templates compiled by its own build.rs. The dependency still builds the
// shared renderer because models and rendering live in the same crate.
sbolt::include_views!();

fn main() {
    let mut context = sbolt::context!(example_views::models::IndexModel {
        name: "sbolt".into(),
        age: 1,
        msg: "Welcome!".into(),
    });
    let (output, _) = cli_standalone_views::render("views/sub/index", None, &mut context)
        .unwrap_or_else(|e| {
            eprintln!("Error: {e:?}");
            std::process::exit(1);
        });
    println!("{output}");
}

#[cfg(test)]
mod tests {
    use super::cli_standalone_views;
    use sbolt::types::template::Kind;

    #[test]
    fn directly_compiled_index_renders_model_fields() {
        let mut context = sbolt::context!(example_views::models::IndexModel {
            name: "standalone".into(),
            age: 1,
            msg: "Welcome!".into(),
        });
        let (output, kind) =
            cli_standalone_views::render("views/sub/index", None, &mut context).unwrap();
        assert_eq!(kind, Kind::KHTML);
        assert!(output.contains("<div>Welcome! - from standalone(1)</div>"));
    }
}
