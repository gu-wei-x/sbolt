#![allow(dead_code)]
use std::path::PathBuf;

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

impl<'a> Template<'a> {
    pub(in crate::codegen::compiler::types) fn generate_code(
        &self,
        view_name: &str,
        view_type: &str,
        full_view_name: &str,
        mod_name: &str,
    ) -> result::Result<TokenStream> {
        let view_name = format_ident!("{}", view_name);
        let view_type = format_ident!("K{}", view_type);
        let template_type = format_ident!("{}", consts::TEMPLATE_TYPE_NAME);
        let imports_content = self.block().generate_imports()?;
        let layout_content = self.block().generate_layout()?;

        // a view must have render method.
        let render_content = self.block().generate_render(mod_name)?;
        let code = quote! {
            use crate::viewtypes::*;
            use disguise::types::Context;
            use disguise::types::Writer;
            #(#imports_content)*

            pub struct #view_name {
                context: disguise::types::DefaultViewContext,
            }

            impl #view_name {
                pub(crate) fn new(context: disguise::types::DefaultViewContext) -> Self {
                    Self {
                        context: context,
                    }
                }

                pub(crate) fn create(context: disguise::types::DefaultViewContext) -> #template_type {
                   #template_type::#view_type(#view_name::new(context))
                }
            }

            impl disguise::types::Template for #view_name
            {
                fn name() -> String {
                    #full_view_name.to_string()
                }

                fn get_data<D: Send + Sync + 'static>(&self, key: &str) -> Option<&D> {
                    self.context.get_data(key)
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

        let code = self.generate_code(
            &view_name,
            &view_type,
            &full_view_name,
            &compiler_options.mod_name,
        )?;
        //fsutil::write_code_to_file(&target, &code)?;
        println!("**********************************");
        println!("{}", code.to_string());
        println!("**********************************");
        Ok(result)
    }
}
