use crate::codegen::compiler;
use crate::codegen::consts;
use crate::utils;

use quote::format_ident;
use quote::quote;
use std::path::PathBuf;

pub(crate) fn generate_view_content(
    file_path: &PathBuf,
    template_path: &PathBuf,
    file_name: &str,
    namespace: &Option<String>,
    compiler_result: &mut compiler::CompileResult,
) -> Result<(), String> {
    let name = file_name;
    let full_name = utils::name::create_full_name(namespace, name);
    let view_name = utils::name::normalize_to_type_name(&name);
    let view_type = utils::name::normalize_to_type_name(&full_name);
    compiler_result.add_view_mapping(full_name.to_string(), view_name.clone());

    let content = std::fs::read_to_string(template_path).unwrap_or_default();
    let view_name = format_ident!("{}", view_name);
    let view_type = format_ident!("K{}", view_type);
    let template_type = format_ident!("{}", consts::TEMPLATE_TYPE_NAME);
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
                #full_name
            }

            fn render(&self, context: &mut impl disguise::types::Context) {
               // TODO: generate code from template
               context.write_line(#content);
            }
        }
    };

    utils::fs::generate_code_with_content(file_path, &view_content)
}
