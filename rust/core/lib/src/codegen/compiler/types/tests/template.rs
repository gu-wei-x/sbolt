#![cfg(test)]
use crate::codegen::types::Template;
use crate::types::result;
use quote::quote;

#[test]
fn template_generate_code() -> result::Result<()> {
    let raw_content = r#"
@use test::test1;
@layout test::test2;
@section test1 {
   this is test1
}
<html>
   <div>Test</div>
</html>"#;
    let template = Template::from(&raw_content, None)?;
    let code = template.generate_code(
        "TestView",
        "TestnsTestViewView",
        "testns::TestView",
        "test_view_mod",
    )?;
    let expected = quote! {
        use crate::viewtypes::*;
        use disguise::types::Context;
        use disguise::types::Writer;
        use test::test1;

        pub struct TestView {
            context: disguise::types::DefaultViewContext,
        }

        impl TestView {
            pub(crate) fn new(context: disguise::types::DefaultViewContext) -> Self {
                Self {
                    context:context,
                }
            }

            pub(crate) fn create(context: disguise::types::DefaultViewContext) -> Template {
                Template::KTestnsTestViewView(TestView::new(context))
            }
        }

        impl disguise::types::Template for TestView
        {
            fn name() -> String {
                "testns::TestView".to_string()
            }

            fn get_data<D: Send + Sync + 'static>(&self, key: &str) -> Option<&D> {
                self.context.get_data(key)
            }

            fn layout() -> Option<String> {
                Some("test::test2".to_string())
            }

            fn render(& self) -> disguise::types::result::RenderResult<String> {
                let mut writer = disguise::types::HtmlWriter::new();
                match Self::layout() {
                    Some(layout) => {
                        for key in disguise::types::resolve_layout_to_view_keys(&layout, &Self::name()) {
                            if let Some(creator) = crate::test_view_mod::resolve_view_creator(& key) {
                                let view = creator(disguise::context!());
                                return view.render();
                            }
                        }
                        Err(disguise::types::error::RuntimeError::layout_not_found(&layout, &Self::name()))
                    }
                    None => Ok(writer.into_string()),
                }
            }
        }
    };
    assert_eq!(code.to_string(), expected.to_string());

    Ok(())
}
