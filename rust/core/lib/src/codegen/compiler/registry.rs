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
        .map(|(name, _view_name)| {
            format!(
                "K{}({}),",
                name::create_view_type_name(&name),
                name::create_type_full_name(name, mode_name)
            )
        })
        .collect::<Vec<String>>()
        .join("\n        ");

    let view_unpack_content = view_name_mapping
        .iter()
        .map(|(name, view_name)| {
            format!(
                "{}::K{}({}) => {}.render(context),",
                consts::TEMPLATE_TYPE_NAME,
                name::create_view_type_name(&name),
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
        use sbolt::types::Template as _;
        pub(crate) enum #type_ident {
            #view_types_ts
        }

        impl #type_ident {
            pub(crate) fn render(&self, context:&mut impl sbolt::types::Context) -> sbolt::types::result::RenderResult<String> {
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
        .map(|(name, _)| {
            let full_type_name = name::create_type_full_name(name, mode_name);
            format!(
                r#"view_reg_creator.insert({}::name(), {}::create);"#,
                full_type_name, full_type_name
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let view_reg_content_ts = view_reg_content
        .parse::<proc_macro2::TokenStream>()
        .unwrap();

    let type_ident = format_ident!("{}", consts::TEMPLATE_TYPE_NAME);
    let content = quote! {
        pub(crate) fn create_view_registrar() -> std::collections::HashMap::<String, fn() -> #type_ident> {
            let mut view_reg_creator = std::collections::HashMap::<String, fn() -> #type_ident>::new();
            // Register views
            #view_reg_content_ts
            view_reg_creator
        }
    };

    Ok(content)
}
