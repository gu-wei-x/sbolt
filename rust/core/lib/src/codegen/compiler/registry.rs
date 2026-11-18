use crate::codegen::compiler::{fsutil, name};
use crate::codegen::{CompilerOptions, consts};
use crate::types::{error::CompileError, template::Kind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) fn generate_registry(
    file_path: &PathBuf,
    view_name_mapping: &HashMap<(String, Kind), String>,
    compiler_option: &CompilerOptions,
) -> Result<(), CompileError> {
    let type_ident = format_ident!("{}", consts::TEMPLATES_TYPE_NAME);
    let mut variants = Vec::new();
    let mut arms = Vec::new();
    let mut registrations = Vec::new();
    for ((key, kind), type_path) in view_name_mapping {
        let variant = format_ident!("K{}", name::create_view_type_name(type_path));
        let full_type = name::create_type_full_name(type_path, compiler_option.mod_name());
        let full_type = syn::parse_str::<TokenStream>(&full_type)?;
        let kind_ident = format_ident!("{kind:?}");
        variants.push(quote! { #variant(#full_type), });
        arms.push(quote! { Self::#variant(view) => view.render(context), });
        registrations.push(quote! {
            view_reg_creator.entry(#key.to_string()).or_default()
                .insert(sbolt::types::template::Kind::#kind_ident, #full_type::create);
        });
    }
    let content = quote! {
        use sbolt::types::Template as _;
        pub(crate) enum #type_ident {
            #(#variants)*
        }

        impl #type_ident {
            pub(crate) fn render(&self, context: &mut impl sbolt::types::Context) -> sbolt::types::result::RenderResult<String> {
                match self {
                    #(#arms)*
                }
            }

            pub(crate) fn create_view_registrar() -> std::collections::HashMap<String, std::collections::HashMap<sbolt::types::template::Kind, fn() -> #type_ident>> {
                let mut view_reg_creator: std::collections::HashMap<String, std::collections::HashMap<sbolt::types::template::Kind, fn() -> #type_ident>> = std::collections::HashMap::new();
                #(#registrations)*
                view_reg_creator
            }
        }
    };
    fsutil::write_code_to_file(file_path, &content)
}
