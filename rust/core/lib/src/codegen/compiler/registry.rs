use crate::codegen::compiler::{fsutil, name};
use crate::codegen::consts;
use crate::types::error::CompileError;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) fn generate_registry(
    file_path: &PathBuf,
    mode_name: &str,
    view_name_mapping: &HashMap<String, String>,
) -> Result<(), CompileError> {
    let view_types_content = view_name_mapping
        .iter()
        .map(|(name, view_name)| {
            format!(
                "K{}(crate::{}::{}::{}),",
                name::normalize_to_type_name(&name),
                mode_name,
                name::create_mode_prefix(&name),
                view_name
            )
        })
        .collect::<Vec<String>>()
        .join("\n        ");

    let view_unpack_content = view_name_mapping
        .iter()
        .map(|(name, view_name)| {
            format!(
                "{}::K{}({}) => {}.render(output),",
                consts::TEMPLATE_TYPE_NAME,
                name::normalize_to_type_name(&name),
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

    let reg_ts = generate_registry_method(mode_name, view_name_mapping)?;
    let type_ident = format_ident!("{}", consts::TEMPLATE_TYPE_NAME);
    let content = quote! {
        use disguise::types::Template as _;
        pub(crate) enum #type_ident {
            #view_types_ts
        }

        impl #type_ident {
            pub(crate) fn render(&self, output: &mut impl disguise::types::Writer) {
                match self {
                   #view_unpack_content_ts
                }
            }

            #reg_ts
        }
    };

    fsutil::write_code_to_file(file_path, &content)
}

fn generate_registry_method(
    mode_name: &str,
    view_name_mapping: &HashMap<String, String>,
) -> Result<TokenStream, String> {
    let view_reg_content = view_name_mapping
        .iter()
        .map(|(name, view_name)| {
            let prefix = format!(
                "crate::{}::{}::{}",
                mode_name,
                name::create_mode_prefix(name),
                view_name
            );
            format!(
                r#"view_reg_creator.insert({}::name(), {}::create);"#,
                prefix, prefix
            )
        })
        .collect::<Vec<String>>()
        .join("\n        ");

    let view_reg_content_ts = view_reg_content
        .parse::<proc_macro2::TokenStream>()
        .unwrap();

    let type_ident = format_ident!("{}", consts::TEMPLATE_TYPE_NAME);
    let content = quote! {
        pub(crate) fn create_view_registrar() -> std::collections::HashMap::<String, fn(context: disguise::types::DefaultViewContext) -> #type_ident> {
            let mut view_reg_creator = std::collections::HashMap::<String, fn(context: disguise::types::DefaultViewContext) -> #type_ident>::new();
            // Register views
            #view_reg_content_ts
            view_reg_creator
        }
    };

    Ok(content)
}
