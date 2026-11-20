sbolt::include_views!();

use bench_views::views::teams_html::TeamsView;
use sbolt::types::Template as _;

pub use bench_views::KTemplate;

pub fn resolve_teams() -> KTemplate {
    bench_views::resolve("views/teams", None).unwrap()
}

pub fn resolve_table() -> KTemplate {
    bench_views::resolve("views/big_table", None).unwrap()
}

pub struct Team {
    pub name: String,
    pub score: i32,
}

pub struct TeamsModel {
    pub teams: Vec<Team>,
}

pub struct TableModel {
    pub rows: Vec<Vec<i32>>,
}

pub struct PageModel {
    pub name: String,
    pub age: i32,
    pub msg: String,
}

pub fn teams_model() -> TeamsModel {
    TeamsModel { teams: team_data() }
}

fn team_data() -> Vec<Team> {
    vec![
        Team {
            name: "Red & Blue".into(),
            score: 17,
        },
        Team {
            name: "Green".into(),
            score: -3,
        },
        Team {
            name: "Gold".into(),
            score: 42,
        },
    ]
}

pub fn teams_context() -> sbolt::types::DefaultViewContext {
    sbolt::context!(model: std::sync::Arc::new(teams_model()))
}

pub fn resolve_teams_model() -> KTemplate {
    resolve_teams()
}

pub fn render_teams_model_named(context: &mut impl sbolt::types::Context) -> String {
    render_teams(context)
}

pub fn render_teams_model(context: &mut impl sbolt::types::Context) -> String {
    TeamsView.render(context).unwrap()
}

pub fn table_context() -> sbolt::types::DefaultViewContext {
    sbolt::context!(model: std::sync::Arc::new(TableModel {
        rows: (0..100)
            .map(|row| (0..100).map(|col| row * 100 + col).collect())
            .collect(),
    }))
}

pub fn render_teams(context: &mut impl sbolt::types::Context) -> String {
    bench_views::render("views/teams", None, context).unwrap().0
}

pub fn render_table(context: &mut impl sbolt::types::Context) -> String {
    bench_views::render("views/big_table", None, context)
        .unwrap()
        .0
}

pub fn resolve_teams_only() -> bool {
    bench_views::resolve_view_creator("views/teams", sbolt::types::template::Kind::KHTML).is_some()
}

pub fn resolve_table_only() -> bool {
    bench_views::resolve_view_creator("views/big_table", sbolt::types::template::Kind::KHTML)
        .is_some()
}

pub fn format_table_values() -> String {
    use sbolt::types::Writer;
    let mut output = String::with_capacity(40_000);
    for value in 0..10_000 {
        Writer::write_formatted(&mut output, format_args!("{}", value));
    }
    output
}

pub fn format_table_values_specialized() -> String {
    use sbolt::types::WriteToRef;
    let mut output = String::with_capacity(40_000);
    for value in 0..10_000 {
        WriteToRef(&(value)).__sbolt_write_to(&mut output);
    }
    output
}

pub fn render_table_with_setup() -> String {
    render_table(&mut table_context())
}

pub fn render_page_with_layout() {
    _ = bench_views::render("views/page_with_layout", None, &mut sbolt::context!());
}

pub fn render_page() {
    let mut context = sbolt::context! {
        model: std::sync::Arc::new(PageModel {
            name: "sbolt".to_string(),
            age: 1,
            msg: "Hello world!".to_string(),
        })
    };
    bench_views::render("views/page", None, &mut context).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_renders_model_fields() {
        let mut context = sbolt::context!(model: std::sync::Arc::new(PageModel {
            name: "sbolt".into(),
            age: 1,
            msg: "Hello world!".into(),
        }));
        let output = bench_views::render("views/page", None, &mut context)
            .unwrap()
            .0;
        assert!(output.contains("<div>Hello world! - from sbolt(1)</div>"));
    }

    #[test]
    fn page_with_layout_renders_sections() {
        let output = bench_views::render("views/page_with_layout", None, &mut sbolt::context!())
            .unwrap()
            .0;
        assert!(output.contains("<title>Home</title>"));
        assert!(output.contains("<li>menu 1</li>"));
        assert!(output.contains("Hello world!"));
        assert!(output.contains("this is footer"));
    }

    #[test]
    fn teams_rendering_paths_preserve_raw_output() {
        let expected = "<ul><li>Red & Blue: 17</li><li>Green: -3</li><li>Gold: 42</li></ul>";
        let mut context = teams_context();
        let named = render_teams(&mut context);
        assert_eq!(named.trim(), expected);
        let handle = resolve_teams();
        assert_eq!(handle.render(&mut context).unwrap(), named);
        context.set_capacity_hint(128);
        assert_eq!(handle.render(&mut context).unwrap(), named);
        let model = std::sync::Arc::new(teams_model());
        let mut typed_context = sbolt::context!(model: std::sync::Arc::clone(&model));
        typed_context.set_capacity_hint(128);
        assert_eq!(render_teams_model_named(&mut typed_context), named);
        assert_eq!(render_teams_model(&mut typed_context), named);
        assert_eq!(
            resolve_teams_model().render(&mut typed_context).unwrap(),
            named
        );
    }

    #[test]
    fn big_table_rendering_paths_preserve_all_cells() {
        use std::fmt::Write as _;
        let mut expected = String::from("<table>");
        for row in 0..100 {
            expected.push_str("<tr>");
            for col in 0..100 {
                write!(expected, "<td>{}</td>", row * 100 + col).unwrap();
            }
            expected.push_str("</tr>");
        }
        expected.push_str("</table>");
        let mut context = table_context();
        let named = render_table(&mut context);
        assert_eq!(named.trim(), expected);
        assert_eq!(render_table_with_setup(), named);
        let handle = resolve_table();
        assert_eq!(handle.render(&mut context).unwrap(), named);
        context.set_capacity_hint(140_000);
        assert_eq!(handle.render(&mut context).unwrap(), named);
        assert_eq!(format_table_values_specialized(), format_table_values());
    }
}
