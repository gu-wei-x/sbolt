use crate::codegen::CompileResult;
use crate::codegen::CompilerOptions;
use crate::codegen::compiler::fsutil;
use crate::codegen::compiler::name;
use crate::codegen::consts;
use crate::codegen::types::Template;
use crate::types::result;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use std::path::PathBuf;

impl<'a> Template<'a> {
    pub(in crate::codegen::compiler::types) fn to_token_stream(
        &self,
        view_name: &str,
        view_type: &str,
        full_view_name: &str,
        mod_name: &str,
    ) -> result::Result<TokenStream> {
        let view_name = format_ident!("{}", view_name);
        let view_type = format_ident!("K{}", view_type);
        let template_type = format_ident!("{}", consts::TEMPLATE_TYPE_NAME);
        let imports_content = self.block().generate_imports_token_stream()?;
        let layout_content = self.block().generate_layout_token_stream()?;
        let kind = match self.kind() {
            crate::types::template::Kind::KHTML => {
                quote! { disguise::types::template::Kind::KHTML }
            }
            crate::types::template::Kind::KJSON => {
                quote! { disguise::types::template::Kind::KJSON }
            }
            crate::types::template::Kind::KTEXT => {
                quote! { disguise::types::template::Kind::KTEXT }
            }
        };
        // a view must have render method.
        let render_content = self.block().generate_render_token_stream(mod_name)?;
        let code = quote! {
            use crate::viewtypes::*;
            use disguise::types::Template as _;
            use disguise::types::Writer;
            #(#imports_content)*

            pub struct #view_name;
            impl #view_name {
                pub(crate) fn new() -> Self {
                    Self {}
                }

                pub(crate) fn create() -> #template_type {
                   #template_type::#view_type(#view_name::new())
                }

                fn create_writer(&self, kind: Option<disguise::types::template::Kind>) -> disguise::types::KWriter {
                    let kind = match kind {
                        Some(k) => k,
                        _ => #view_name::kind(),
                    };
                    match kind {
                        disguise::types::template::Kind::KHTML => {
                            disguise::types::KWriter::KHtml(disguise::types::HtmlWriter::new())
                        },
                        _ => disguise::types::KWriter::KText(String::new()),
                    }
                }
            }

            impl disguise::types::Template for #view_name
            {
                fn name() -> String {
                    #full_view_name.to_string()
                }

                fn kind() -> disguise::types::template::Kind {
                     #kind
                }

                #layout_content

                #render_content
            }
        };
        Ok(code)
    }

    pub(in crate::codegen::compiler) fn compile(
        &self,
        target: PathBuf,
        compiler_options: &CompilerOptions,
    ) -> result::Result<CompileResult> {
        let name = fsutil::get_file_name(&target).ok_or(format!(
            "Failed to get directory name from {}",
            target.display()
        ))?;

        let mut result = CompileResult::default();
        let view_name = name::create_view_type_name(&name);
        let namespace = self.namespace().cloned();
        let namespace = name::create_name_space(&namespace, &name);
        let full_view_name = name::create_normalized_name(&Some(namespace), &view_name);
        let view_type = name::create_view_type_name(&full_view_name);
        result.add_view_mapping(full_view_name.to_string(), view_name.clone());

        let code = self.to_token_stream(
            &view_name,
            &view_type,
            &full_view_name,
            &compiler_options.mod_name,
        )?;
        fsutil::write_code_to_file(&target, &code)?;
        Ok(result)
    }
}
