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
use syn::Ident;

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
                quote! { sbolt::types::template::Kind::KHTML }
            }
            crate::types::template::Kind::KJSON => {
                quote! { sbolt::types::template::Kind::KJSON }
            }
            crate::types::template::Kind::KTEXT => {
                quote! { sbolt::types::template::Kind::KTEXT }
            }
        };
        // a view must have render method.
        let render_content = self.block().generate_render_token_stream(mod_name)?;
        let code = quote! {
            use crate::viewtypes::*;
            use sbolt::types::Template as _;
            use sbolt::types::Writer;
            #(#imports_content)*

            pub struct #view_name;
            impl #view_name {
                pub(crate) fn new() -> Self {
                    Self {}
                }

                pub(crate) fn create() -> #template_type {
                   #template_type::#view_type(#view_name::new())
                }

                fn create_writer(&self, kind: Option<sbolt::types::template::Kind>) -> sbolt::types::KWriter {
                    let kind = match kind {
                        Some(k) => k,
                        _ => #view_name::kind(),
                    };
                    match kind {
                        sbolt::types::template::Kind::KHTML => {
                            sbolt::types::KWriter::KHtml(sbolt::types::HtmlWriter::new())
                        },
                        _ => sbolt::types::KWriter::KText(String::new()),
                    }
                }
            }

            impl sbolt::types::Template for #view_name
            {
                fn name() -> String {
                    #full_view_name.to_string()
                }

                fn kind() -> sbolt::types::template::Kind {
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
        syn::parse_str::<Ident>(&view_name).map_err(|_| {
            format!(
                "'{}' is not a valid ident name, please change the file name",
                view_name
            )
        })?;
        syn::parse_str::<Ident>(&compiler_options.mod_name).map_err(|_| {
            format!(
                "'{}' is not a valid ident name, please change the mod name",
                compiler_options.mod_name
            )
        })?;
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
