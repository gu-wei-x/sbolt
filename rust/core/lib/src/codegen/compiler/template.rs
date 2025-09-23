use crate::{
    codegen::{CompileResult, consts, parser::template::Template},
    utils,
};
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use std::path::PathBuf;

impl<'a> Template<'a> {
    pub(crate) fn compile(&self, target: PathBuf) -> CompileResult {
        let mut result = CompileResult::default();
        match utils::fs::get_file_name(&target) {
            None => {
                return result;
            }
            Some(name) => {
                let full_view_name = utils::name::create_full_name(&self.namespace, &name);
                let view_name = utils::name::normalize_to_type_name(&name);
                let view_type = utils::name::normalize_to_type_name(&full_view_name);
                result.add_view_mapping(full_view_name.to_string(), view_name.clone());

                let view_name = format_ident!("{}", view_name);
                let view_type = format_ident!("K{}", view_type);
                let template_type = format_ident!("{}", consts::TEMPLATE_TYPE_NAME);

                let mut render_content = String::new();
                self.generate_code(&mut render_content);
                let render_content_ts: TokenStream = render_content.parse().unwrap();
                let view_content = quote! {
                    use crate::viewtypes::*;

                    pub struct #view_name;
                    impl #view_name {
                        pub(crate) fn new() -> Self {
                            Self
                        }

                        pub(crate) fn create() -> #template_type {
                           #template_type::#view_type(#view_name::new())
                        }
                    }

                    impl disguise::types::Template for #view_name
                    {
                        fn name() -> &'static str {
                            #full_view_name
                        }

                        fn render(&self, context: &mut impl disguise::types::Context) {
                           #render_content_ts
                        }
                    }
                };

                _ = utils::fs::generate_code_with_content(&target, &view_content);
            }
        }

        result
    }

    pub(crate) fn generate_code(&self, output: &mut String) {
        for block in &self.blocks {
            block.generate_code(&None, output);
        }
    }
}
