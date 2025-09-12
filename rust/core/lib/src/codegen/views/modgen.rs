use crate::utils;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) fn generate_mod_content(
    mod_file: &PathBuf,
    mod_name: &str,
    imported_mods: &[String],
    _view_name_mapping: &HashMap<String, String>,
) -> Result<(), String> {
    let mod_name = format_ident!("{}", mod_name);
    let imported_content: String = imported_mods
        .iter()
        .map(|m| format!("mod {};\n", m))
        .collect::<String>();
    let re_exported_content: String = imported_mods
        .iter()
        .map(|m| format!("pub use super::{}::*;\n", m))
        .collect::<String>();

    let imported_mods: TokenStream = imported_content.parse().unwrap();
    let re_exported_mods: TokenStream = re_exported_content.parse().unwrap();

    let content = quote! {
        #imported_mods

        #[allow(unused_imports)]
        pub mod #mod_name {
            #re_exported_mods

            // TemplateResolver.
            struct TemplateResolver {
                view_creators: std::collections::HashMap<&'static str, fn() -> crate::views::Views>,
            }

            impl TemplateResolver {
                fn new() -> Self {
                    Self {
                        view_creators: crate::views::Views::create_view_registrar(),
                    }
                }

                fn resolve(&self, name: &str) -> Option<crate::views::Views> {
                    let template_name = disguise::utils::fs::normalize_name(name, "", "");
                    self.view_creators.get(template_name.as_str()).map(|f| f())
                }
            }

            static TEMPLATE_RESOLVER: std::sync::LazyLock<TemplateResolver> = std::sync::LazyLock::new(|| {
                TemplateResolver::new()
            });

            pub(crate) fn render(name: &str, context: &mut impl disguise::types::Context) {
                 if let Some(view) = TEMPLATE_RESOLVER.resolve(name) {
                    view.render(context);
                 }
            }
        }
    };

    utils::fs::generate_code_with_content(mod_file, &content)
}
