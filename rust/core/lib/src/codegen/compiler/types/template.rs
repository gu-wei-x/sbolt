use crate::codegen::CompileResult;
use crate::codegen::CompilerOptions;
use crate::codegen::StaticOutputStats;
use crate::codegen::compiler::context::CodeGenContext;
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
        compiler_options: &CompilerOptions,
    ) -> result::Result<(TokenStream, StaticOutputStats)> {
        let view_name = format_ident!("{}", view_name);
        let view_type = format_ident!("K{}", view_type);
        let template_type = format_ident!("{}", consts::TEMPLATES_TYPE_NAME);
        let template_type_full_name = name::create_type_full_name(
            &format!(
                "{}::{}",
                consts::TEMPLATES_MAP_FILE_NAME,
                consts::TEMPLATES_TYPE_NAME
            ),
            compiler_options.mod_name(),
        );
        let template_type_ts = syn::parse_str::<TokenStream>(&template_type_full_name)?;
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
        let mut context = CodeGenContext::new(self.kind(), compiler_options);
        let render_content = self.block().generate_render_token_stream(&mut context)?;
        let code = quote! {
            use #template_type_ts;
            use sbolt::types::Template as _;
            use sbolt::types::Writer;
            #[allow(unused_imports)]
            use sbolt::types::{DisplayWriteTo as _, OptionDisplayWriteTo as _};
            #(#imports_content)*

            #[derive(Clone, Copy)]
            pub struct #view_name;
            impl #view_name {
                pub(crate) fn new() -> Self {
                    Self {}
                }

                pub(crate) fn create() -> #template_type {
                   #template_type::#view_type(#view_name::new())
                }

                fn create_writer(
                    &self,
                    kind: Option<sbolt::types::template::Kind>,
                    capacity_hint: Option<usize>,
                ) -> sbolt::types::KWriter {
                    let kind = match kind {
                        Some(k) => k,
                        _ => #view_name::kind(),
                    };
                    match kind {
                        sbolt::types::template::Kind::KHTML => {
                            sbolt::types::KWriter::KHtml(sbolt::types::HtmlWriter::new(capacity_hint))
                        },
                        _ => sbolt::types::KWriter::KText(String::with_capacity(capacity_hint.unwrap_or_default())),
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
        Ok((code, context.static_output_stats()))
    }

    pub(in crate::codegen::compiler) fn compile(
        &self,
        source_name: &str,
        target: PathBuf,
        compiler_options: &CompilerOptions,
    ) -> result::Result<CompileResult> {
        let name = fsutil::get_file_name(&target).ok_or(format!(
            "Failed to get directory name from {}",
            target.display()
        ))?;
        let mut result = CompileResult::default();
        let view_name = name::create_view_type_name(source_name);
        let namespace = self.namespace().cloned();
        let logical_namespace = name::create_name_space(&namespace, source_name);
        let full_view_name = crate::types::normalize_path_to_view_key(&logical_namespace)
            .ok_or("Failed to create a template lookup key")?;
        let module_namespace = name::create_name_space(&namespace, &name);
        let type_path = name::create_name_space(&Some(module_namespace), &view_name);
        let view_type = name::create_view_type_name(&type_path);
        result.add_view_mapping(full_view_name.to_string(), self.kind(), type_path)?;
        syn::parse_str::<Ident>(&view_name).map_err(|_| {
            format!(
                "'{}' is not a valid ident name, please change the file name",
                view_name
            )
        })?;
        syn::parse_str::<Ident>(compiler_options.mod_name()).map_err(|_| {
            format!(
                "'{}' is not a valid ident name, please change the mod name",
                compiler_options.mod_name()
            )
        })?;
        let (code, stats) =
            self.to_token_stream(&view_name, &view_type, &full_view_name, compiler_options)?;
        fsutil::write_code_to_file(&target, &code)?;
        result.add_static_output_stats(stats);
        Ok(result)
    }
}
