#![cfg(test)]
use crate::codegen::types::Template;
use crate::codegen::{self, CompilerOptions};
use crate::types::result;
use crate::types::template::Kind;
use quote::quote;

#[test]
fn malformed_model_blocks_report_errors_during_generation() -> result::Result<()> {
    let options = CompilerOptions::default();
    for source in [
        "@model Vec<;\nbody",
        "@model layout:;\nbody",
        "@model 123: String;",
        "@model \"layout\": String;",
        "@model : String;",
    ] {
        let template = Template::from(source, None, Kind::KHTML, &options)?;
        let error = template
            .to_token_stream("PageView", "ViewsPageView", "views/page", &options)
            .unwrap_err();
        assert!(
            error.to_string().contains("Invalid model declaration"),
            "{source}: {error}"
        );
    }
    Ok(())
}

#[test]
fn duplicate_model_blocks_report_errors_during_generation() -> result::Result<()> {
    let options = CompilerOptions::default();
    for source in [
        "@model String;\n@model usize;\nbody",
        "@model page: String;\n@model layout: usize;",
    ] {
        let template = Template::from(source, None, Kind::KHTML, &options)?;
        let error = template
            .to_token_stream("PageView", "ViewsPageView", "views/page", &options)
            .unwrap_err();
        assert!(
            error.to_string().contains("Multiple model directives"),
            "{error}"
        );
    }
    Ok(())
}

#[test]
fn model_rendering_does_not_generate_a_helper() -> result::Result<()> {
    let options = CompilerOptions::default();
    let template = Template::from("@model String;\n@(model)", None, Kind::KTEXT, &options)?;
    let (tokens, _) =
        template.to_token_stream("PageView", "ViewsPageView", "views/page", &options)?;
    let file = syn::parse2::<syn::File>(tokens)?;
    let has_model_helper = file
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Impl(item) if item.trait_.is_none() => Some(item),
            _ => None,
        })
        .flat_map(|item| &item.items)
        .any(
            |item| matches!(item, syn::ImplItem::Fn(method) if method.sig.ident == "render_model"),
        );
    assert!(!has_model_helper);
    Ok(())
}

#[test]
fn model_directives_do_not_render_or_disable_static_json_optimization() -> result::Result<()> {
    let options = CompilerOptions::default().with_optimization(true);
    for source in [
        "@model String;\n{ \"count\": 1 }",
        "@model layout: String;\n{ \"count\": 1 }",
    ] {
        let template = Template::from(source, None, Kind::KJSON, &options)?;
        let (tokens, stats) =
            template.to_token_stream("PageView", "ViewsPageView", "views/page", &options)?;
        let write = quote! { writer.write("{\"count\":1}"); };
        assert!(tokens.to_string().contains(&write.to_string()));
        assert_eq!(stats.output_bytes(), "{\"count\":1}".len());
    }
    Ok(())
}

#[test]
fn to_token_stream_html() -> result::Result<()> {
    let raw_content = r#"
@use test::test1;
@layout test::test2;
@section test1 {@{
   let name = "test1";
}
this is @name}
<html><div>Test</div></html>"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let (ts, _) = template.to_token_stream(
        "TestView",
        "TestnsTestViewView",
        "testns::TestView",
        &CompilerOptions::default().with_mod_name("test_view_mod"),
    )?;
    let expected = quote! {
     use crate::test_view_mod::ktemplate::KTemplate;
     use sbolt::types::Template as _;
     use sbolt::types::Writer;
     use test::test1;
     pub struct TestView;

     impl TestView {
         pub(crate) fn new() -> Self {
             Self {}
         }

         pub(crate) fn create() -> KTemplate {
             KTemplate::KTestnsTestViewView(TestView::new())
         }

         fn create_writer(&self, kind: Option<sbolt::types::template::Kind>) -> sbolt::types::KWriter {
             let kind = match kind {
                 Some(k) => k,
                 _ => TestView::kind(),
             };
             match kind {
                 sbolt::types::template::Kind::KHTML => {
                     sbolt::types::KWriter::KHtml(sbolt::types::HtmlWriter::new())
                 },
                 _ => sbolt::types::KWriter::KText(String::new()),
             }
         }
     }
     impl sbolt::types::Template for TestView {
         fn name() -> String {
             "testns::TestView".to_string()
         }

         fn kind() -> sbolt::types::template::Kind {
             sbolt::types::template::Kind::KHTML
         }

         fn layout() -> Option<String> {
             Some("test::test2".to_string())
         }

         fn render(&self, #[allow(unused_variables)]context: &mut impl sbolt::types::Context) -> sbolt::types::result::RenderResult<String> {
             let mut writer = self.create_writer(None);
             let section_name = "test1";
             let section_writer = {
                 let mut writer = self.create_writer(None);
                 let name = "test1";
                 writer.write("this is ");
                 writer.write(&name.to_string());
                 writer
             };
             context.add_section(section_name, section_writer.into_string());
             writer.write("<html><div>Test</div></html>");
             match Self::layout() {
                 Some(layout) => {
                     for key in sbolt::types::resolve_layout_to_view_keys(&layout, &Self::name()) {
                         if let Some(creator) = crate::test_view_mod::resolve_view_creator(&key, Self::kind()) {
                             context.set_default_section(writer.into_string());
                             let view = creator();
                             return view.render(context);
                         }
                     }
                     Err(sbolt::types::error::RuntimeError::layout_not_found(&layout, &Self::name()))
                 }
                 None => Ok(writer.into_string()),
             }
         }
     }
    };
    assert_eq!(ts.to_string(), expected.to_string());
    Ok(())
}

#[test]
fn to_token_stream_html_without_layout() -> result::Result<()> {
    let raw_content = r#"
@use test::test1;
<html><div>Test</div></html>"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let (ts, _) = template.to_token_stream(
        "TestView",
        "TestnsTestViewView",
        "testns::TestView",
        &CompilerOptions::default().with_mod_name("test_view_mod"),
    )?;
    let expected = quote! {
     use crate::test_view_mod::ktemplate::KTemplate;
     use sbolt::types::Template as _;
     use sbolt::types::Writer;
     use test::test1;
     pub struct TestView;

     impl TestView {
         pub(crate) fn new() -> Self {
             Self {}
         }

         pub(crate) fn create() -> KTemplate {
             KTemplate::KTestnsTestViewView(TestView::new())
         }

         fn create_writer(&self, kind: Option<sbolt::types::template::Kind>) -> sbolt::types::KWriter {
             let kind = match kind {
                 Some(k) => k,
                 _ => TestView::kind(),
             };
             match kind {
                 sbolt::types::template::Kind::KHTML => {
                     sbolt::types::KWriter::KHtml(sbolt::types::HtmlWriter::new())
                 },
                 _ => sbolt::types::KWriter::KText(String::new()),
             }
         }
     }
     impl sbolt::types::Template for TestView {
         fn name() -> String {
             "testns::TestView".to_string()
         }

         fn kind() -> sbolt::types::template::Kind {
             sbolt::types::template::Kind::KHTML
         }

         fn render(&self, #[allow(unused_variables)]context: &mut impl sbolt::types::Context) -> sbolt::types::result::RenderResult<String> {
             let mut writer = self.create_writer(None);
             writer.write("<html><div>Test</div></html>");
             Ok (writer.into_string())
         }
     }
    };
    assert_eq!(ts.to_string(), expected.to_string());
    Ok(())
}

#[test]
fn to_token_stream_json() -> result::Result<()> {
    let raw_content = r#"
@use test::test1;
@layout test::test2;
@section test1 {@{
   let name = "test1";
}
this is @name}"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KJSON, &options)?;
    let ts = template.to_token_stream(
        "TestView",
        "TestnsTestViewView",
        "testns::TestView",
        &CompilerOptions::default().with_mod_name("test_view_mod"),
    );
    assert!(ts.is_ok());
    Ok(())
}

#[test]
fn to_token_stream_txt() -> result::Result<()> {
    let raw_content = r#"
@use test::test1;
@layout test::test2;
@section test1 {@{
   let name = "test1";
}
this is @name}"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KTEXT, &options)?;
    let ts = template.to_token_stream(
        "TestView",
        "TestnsTestViewView",
        "testns::TestView",
        &CompilerOptions::default().with_mod_name("test_view_mod"),
    );
    assert!(ts.is_ok());
    Ok(())
}

#[test]
#[should_panic]
fn compile_without_invalid_file_name() {
    let raw_content = r#"test"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();

    let option = codegen::CompilerOptions::default();
    template
        .compile("te-st", "te-st.rshtml".into(), &option)
        .expect("Expect valid ident from file name");
}

#[test]
#[should_panic]
fn compile_without_invalid_mod_name() {
    let raw_content = r#"test"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options);
    assert!(template.is_ok());
    let template = template.unwrap();

    let option = codegen::CompilerOptions::default().with_mod_name("test-mod_name");
    template
        .compile("test", "test.rshtml".into(), &option)
        .expect("Expect valid ident from mod name");
}
