use sbolt::types::{Context, result::RenderResult, template::Kind};

// Keep model paths identical for shared and directly compiled templates.
extern crate self as example_views;

pub mod models;

sbolt::include_views!();

// The generated renderer is pub(crate), so a wrapper exposes it to other crates.
pub fn render(
    name: &str,
    kind: Option<Kind>,
    context: &mut impl Context,
) -> RenderResult<(String, Kind)> {
    shared_views::render(name, kind, context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_html_preserves_layout_and_escapes_model_input() -> RenderResult<()> {
        let model = models::WelcomeModel {
            name: "<script>&\"'".into(),
        };
        let (output, kind) = render(
            "views/welcome",
            Some(Kind::KHTML),
            &mut sbolt::context!(model),
        )?;
        assert_eq!(kind, Kind::KHTML);
        let head = if cfg!(debug_assertions) {
            "<html>\n    <head>"
        } else {
            "<html><head>"
        };
        assert_eq!(
            output,
            format!(
                "{head}\n        <title>Welcome</title>\n    </head>\n    <body>\n        <div>Welcome: &lt;script&gt;&amp;&quot;&#39;!</div>\n    </body>\n</html>"
            )
        );
        Ok(())
    }

    #[test]
    fn index_html_renders_complete_model_output() -> RenderResult<()> {
        let model = models::IndexModel {
            name: "<team>".into(),
            age: 42,
            msg: "Hello & welcome".into(),
        };
        let (output, kind) = render(
            "views/sub/index",
            Some(Kind::KHTML),
            &mut sbolt::context!(model),
        )?;
        assert_eq!(kind, Kind::KHTML);
        let head = if cfg!(debug_assertions) {
            "<html>\n    <head>"
        } else {
            "<html><head>"
        };
        assert_eq!(
            output,
            format!(
                "{head}\n        <title>Welcome</title>\n    </head>\n    <body>\n        <div>Hello &amp; welcome - from &lt;team&gt;(42)</div>\n    </body>\n</html>"
            )
        );
        Ok(())
    }

    #[test]
    fn welcome_json_serializes_model_input() -> RenderResult<()> {
        let name = "quoted \"name\"\n<script>";
        let (output, kind) = render(
            "views/welcome",
            Some(Kind::KJSON),
            &mut sbolt::context!(models::WelcomeModel { name: name.into() }),
        )?;
        assert_eq!(kind, Kind::KJSON);
        assert_eq!(
            output,
            "{\n  \"framework\": \"quoted \\\"name\\\"\\n\\u003cscript\\u003e\",\n  \"message\": \"Welcome!\"\n}\n"
        );
        Ok(())
    }
}
