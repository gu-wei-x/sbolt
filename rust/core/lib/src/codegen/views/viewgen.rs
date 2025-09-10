use crate::utils;
use quote::format_ident;
use quote::quote;
use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) fn generate_view_content(
    file_path: &PathBuf,
    base_path: &PathBuf,
    template_path: &PathBuf,
    view_name_mapping: &mut HashMap<String, String>,
) -> Result<(), String> {
    let name = utils::fs::path_to_name(&base_path, template_path, "", ".rs");
    let name = name.strip_suffix(".rs").unwrap();
    let view_name = utils::name::normalize_path_to_view_name(&name);
    view_name_mapping.insert(name.to_string(), view_name.clone());

    let content = std::fs::read_to_string(template_path).unwrap_or_default();
    let view_name = format_ident!("{}", view_name);
    let view_content = quote! {
        pub struct #view_name;
        impl #view_name {
            pub fn new() -> Self {
                Self
            }
        }

        impl disguise::types::Template for #view_name
        {
            fn render(&self, context: &mut disguise::types::ViewContext<dyn disguise::types::Writer>) {
                context.write(#content);
            }
        }
    };

    utils::fs::generate_code_with_content(file_path, &view_content)
}
