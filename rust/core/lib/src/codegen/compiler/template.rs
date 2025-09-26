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

                let imports_content = self.generate_imports();
                let layout_content = self.generate_layout();
                let mut render_content = String::new();
                self.generate_code(&mut render_content);
                let render_content_ts: TokenStream = render_content.parse().unwrap();
                let view_content = quote! {
                    use crate::viewtypes::*;
                    #imports_content

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
                        fn name() -> String {
                            #full_view_name.to_string()
                        }

                        #layout_content

                        #[allow(unused_variables)]
                        fn render(&self, context: impl disguise::types::Context, output: &mut impl disguise::types::Writer) {
                            #render_content_ts
                        }
                    }
                };

                _ = utils::fs::generate_code_with_content(&target, &view_content);
            }
        }

        result
    }

    fn generate_code(&self, output: &mut String) {
        for block in &self.blocks {
            if block.name.is_none() {
                block.generate_code(&None, output);
            }
        }
    }

    fn generate_imports(&self) -> Option<TokenStream> {
        let import_content = self
            .blocks
            .iter()
            .map(|block| {
                if block.name == Some(consts::DIRECTIVE_KEYWORD_USE.to_string()) {
                    let import_content = block.content();
                    format!("{} {};", consts::DIRECTIVE_KEYWORD_USE, import_content)
                } else {
                    "".to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("");

        let ts: TokenStream = import_content.parse::<TokenStream>().unwrap();
        Some(quote! {
            #ts
        })
    }

    fn generate_layout(&self) -> TokenStream {
        let items = self
            .blocks
            .iter()
            .filter(|b| b.name == Some("layout".to_string()))
            .collect::<Vec<_>>();
        let layout_count = items.len();
        match layout_count {
            1 => {
                let layout_block = items[0];
                let layout_name = layout_block.content();
                quote! {
                    fn layout() -> Option<String> {
                       Some(#layout_name.to_string())
                    }
                }
            }
            _ => quote! {
                fn layout() -> Option<String> {
                    None
                }
            },
        }
    }
}
