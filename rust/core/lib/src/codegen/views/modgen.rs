use crate::codegen::consts;
use crate::utils;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use std::path::PathBuf;

pub(crate) fn generate_root_mod_file(
    mod_file: &PathBuf,
    mod_name: &str,
    imported_mods: &[String],
) -> Result<(), String> {
    let viewtypes_ident = format!(
        "{}::{}",
        consts::TEMPLATES_MAP_FILE_NAME,
        consts::TEMPLATE_TYPE_NAME
    );

    let mod_name = format_ident!("{}", mod_name);
    let import_content: String = imported_mods
        .iter()
        .map(|m| format!("mod {};\n", m))
        .collect::<String>();
    let re_export_content: String = imported_mods
        .iter()
        .map(|m| {
            format!(
                "pub(crate) mod {} {{ pub(crate) use super::super::{}::*; }}\n",
                m, m
            )
        })
        .collect::<String>();

    let import_content_ts: TokenStream = import_content.parse().unwrap();
    let re_export_content_ts: TokenStream = re_export_content.parse().unwrap();
    let viewtypes_ident_ts: TokenStream = viewtypes_ident.parse().unwrap();

    let content = quote! {
        #import_content_ts
        pub mod #mod_name {
            #re_export_content_ts

            // TemplateResolver.
            struct TemplateResolver {
                view_creators: std::collections::HashMap<&'static str, fn() -> #viewtypes_ident_ts>,
            }

            impl TemplateResolver {
                fn new() -> Self {
                    Self {
                        view_creators: #viewtypes_ident_ts::create_view_registrar(),
                    }
                }

                fn resolve(&self, name: &str) -> Option<#viewtypes_ident_ts> {
                    let normalized_name: &str = &disguise::utils::name::normalize_name(name);
                    self.view_creators.get(normalized_name).map(|f| f())
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

pub(crate) fn generate_sub_mod_file(mod_file: &PathBuf, imported_mods: &[String]) {
    let imported_content: String = imported_mods
        .iter()
        .map(|m| format!("pub(crate) mod {};\n", m))
        .collect::<String>();

    let imported_mods: TokenStream = imported_content.parse().unwrap();
    let content = quote! {
        #imported_mods
    };

    // Handle the Result, e.g., by unwrapping or logging the error
    _ = utils::fs::generate_code_with_content(mod_file, &content);
}
