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
    view_name_mapping: &HashMap<String, String>,
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

    println!("Imported mods: {}", imported_content);
    println!("Re-exported mods: {}", re_exported_content);
    let imported_mods: TokenStream = imported_content.parse().unwrap();
    let re_exported_mods: TokenStream = re_exported_content.parse().unwrap();

    let view_registration_content = view_name_mapping
        .iter()
        .map(|(name, view_name)| {
            format!(
                r#"view_engine.view_map.insert(
            "{}".into(),
            Box::new(|/*todo: context*/| Box::new(self::{}::new())),
        );"#,
                name, view_name
            )
        })
        .collect::<Vec<String>>()
        .join("\n        ");

    let view_registration = view_registration_content.parse::<TokenStream>().unwrap();
    let content = quote! {
        #imported_mods

        #[allow(unused_imports)]
        pub mod #mod_name {
            #re_exported_mods

            // ViewEngine.
            struct ViewEngine {
                view_map: std::collections::HashMap<String, Box<dyn Fn() -> Box<dyn disguise::template::Template> + Send + Sync>>,
            }

            impl ViewEngine {
                fn new() -> Self {
                    let mut view_engine = ViewEngine {
                        view_map: std::collections::HashMap::new(),
                    };

                    #view_registration
                    view_engine
                }

                fn get_view(&self, name: &str) -> Option<&Box<dyn Fn() -> Box<dyn disguise::template::Template> + Send + Sync>> {
                    self.view_map.get(name)
                }
            }

            static VIEWS: std::sync::LazyLock<ViewEngine> = std::sync::LazyLock::new(|| {
                ViewEngine::new()
            });

            pub fn get_view(name: &str) -> Option<Box<dyn disguise::template::Template>> {
                let view_name = disguise::utils::fs::normalize_name(name, "", "");
                let view = self::VIEWS.get_view(&view_name);
                if let Some(view_fn) = view {
                    return Some(view_fn());
                }
                None
            }
        }
    };

    utils::fs::generate_code_with_content(mod_file, &content)
}
