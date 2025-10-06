#![cfg(test)]
use crate::codegen::types::Block;
use crate::codegen::types::Span;
use crate::codegen::types::Template;
use crate::types::result;
use quote::quote;

#[test]
fn generate_code_for_empty_block() -> result::Result<()> {
    let span = Span::new("");
    let block = Block::new_root(span);
    let template = Template::new(Some("testns".to_string()), block);
    let code = template.generate_code("test")?;
    let expected = quote! {
        use crate::viewtypes::*;
        use disguise::types::Context;
        use disguise::types::Writer;

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
        }
    };
    println!("********************************");
    println!("{}", code.to_string());
    println!("********************************");
    assert_eq!(code.to_string(), expected.to_string());
    Ok(())
}
