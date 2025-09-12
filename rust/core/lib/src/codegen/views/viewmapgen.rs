use crate::utils;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) fn generate_view_map(
    file_path: &PathBuf,
    crate_name: &str,
    view_name_mapping: &HashMap<String, String>,
) -> Result<(), String> {
    let view_types_content = view_name_mapping
        .iter()
        .map(|(_name, view_name)| format!(r#"K{}({}),"#, view_name, view_name))
        .collect::<Vec<String>>()
        .join("\n        ");

    let view_unpack_content = view_name_mapping
        .iter()
        .map(|(_name, view_name)| {
            format!(
                r#"Views::K{}({}) => {}.render(context),"#,
                view_name,
                view_name.to_lowercase(),
                view_name.to_lowercase()
            )
        })
        .collect::<Vec<String>>()
        .join("\n        ");

    let view_types_ts = view_types_content
        .parse::<proc_macro2::TokenStream>()
        .unwrap();
    let view_unpack_content_ts = view_unpack_content
        .parse::<proc_macro2::TokenStream>()
        .unwrap();

    let crate_name_ts = format_ident!("{}", crate_name);
    let reg_ts = generate_view_registration(view_name_mapping)?;
    let content = quote! {
        use crate::#crate_name_ts::*;
        use disguise::types::Template;

        pub(crate) enum Views {
            #view_types_ts
        }

        impl Views {
            pub(crate) fn render(&self, context: &mut impl disguise::types::Context) {
                match self {
                   #view_unpack_content_ts
                   // _ => {}
                }
            }

            #reg_ts
        }
    };

    utils::fs::generate_code_with_content(file_path, &content)
}

fn generate_view_registration(
    view_name_mapping: &HashMap<String, String>,
) -> Result<TokenStream, String> {
    let view_reg_content = view_name_mapping
        .iter()
        .map(|(_name, view_name)| {
            format!(
                r#"view_reg_creator.insert({}::name(), {}::create);"#,
                view_name, view_name,
            )
        })
        .collect::<Vec<String>>()
        .join("\n        ");

    let view_reg_content_ts = view_reg_content
        .parse::<proc_macro2::TokenStream>()
        .unwrap();

    let content = quote! {
        pub(crate) fn create_view_registrar() -> std::collections::HashMap::<&'static str, fn()->Views> {
            let mut view_reg_creator = std::collections::HashMap::<&'static str, fn()->Views>::new();
            // Register views
            #view_reg_content_ts
            view_reg_creator
        }
    };

    Ok(content)
}
