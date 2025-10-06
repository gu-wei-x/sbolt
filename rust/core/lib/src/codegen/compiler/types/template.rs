#![allow(dead_code)]
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
        name: &str,
    ) -> result::Result<TokenStream> {
        let view_name = name::create_view_type_name(&name);
        let namespace = self.namespace().cloned();
        let full_view_name = name::create_normalized_name(&namespace, &view_name);
        let view_type = name::create_view_type_name(&full_view_name);

        let view_name = format_ident!("{}", view_name);
        let view_type = format_ident!("K{}", view_type);
        let template_type = format_ident!("{}", consts::TEMPLATE_TYPE_NAME);
        let imports_content: Option<TokenStream> = None;
        let layout_content: Option<TokenStream> = None;
        let render_content: Option<TokenStream> = None;
        let code = quote! {
            use crate::viewtypes::*;
            use disguise::types::Context;
            use disguise::types::Writer;
            #imports_content

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
}
