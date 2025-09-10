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

    let imported_mods: TokenStream = imported_content.parse().unwrap();
    let re_exported_mods: TokenStream = re_exported_content.parse().unwrap();
    let view_registration_content = view_name_mapping
        .iter()
        .map(|(name, view_name)| {
            format!(
                r#"view_resolver.template_creators.insert(
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

            // DefaultTemplateResolver.
            struct DefaultTemplateResolver {
                template_creators: std::collections::HashMap<String, Box<dyn Fn() -> Box<dyn disguise::types::Template> + Send + Sync>>,
            }

            impl DefaultTemplateResolver {
                fn new() -> Self {
                    let mut view_resolver = DefaultTemplateResolver {
                        template_creators: std::collections::HashMap::new(),
                    };

                    #view_registration
                    view_resolver
                }

                fn get_view_creator(&self, name: &str) -> Option<&Box<dyn Fn() -> Box<dyn disguise::types::Template> + Send + Sync>> {
                    self.template_creators.get(name)
                }
            }

            impl disguise::types::TemplateResolver for DefaultTemplateResolver {
                fn resolve(&self, name: &str) -> Option<Box<dyn disguise::types::Template>> {
                    if let Some(view_fn) = self.get_view_creator(name) {
                        return Some(view_fn());
                    }
                    None
                }
            }

            static TEMPLATE_RESOLVER: std::sync::LazyLock<DefaultTemplateResolver> = std::sync::LazyLock::new(|| {
                DefaultTemplateResolver::new()
            });

            fn get_template(name: &str) -> Option<Box<dyn disguise::types::Template>> {
                let template_name = disguise::utils::fs::normalize_name(name, "", "");
                let template_fn = TEMPLATE_RESOLVER.get_view_creator(&template_name);
                if let Some(template_fn) = template_fn {
                    return Some(template_fn());
                }
                None
            }

            pub fn render(name: &str, context: &mut disguise::types::ViewContext<dyn disguise::types::Writer>) {
                 if let Some(template) = get_template(name) {
                     template.render(context);
                 }
            }
        }
    };

    utils::fs::generate_code_with_content(mod_file, &content)
}
