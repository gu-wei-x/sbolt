use crate::{
    codegen::{CompileResult, consts, parser::template::Template},
    utils,
};
use quote::format_ident;
use quote::quote;
use std::path::PathBuf;

impl<'a> Template<'a> {
    pub(crate) fn compile(&self, target: PathBuf) -> CompileResult {
        let mut result = CompileResult::default();
        match utils::fs::get_file_name(&target) {
            None => {
                return result;
            }
            Some(name) => {
                let namespace = self.namespace().cloned();
                let full_view_name = utils::name::create_full_name(&namespace, &name);
                let view_name = utils::name::normalize_to_type_name(&name);
                let view_type = utils::name::normalize_to_type_name(&full_view_name);
                result.add_view_mapping(full_view_name.to_string(), view_name.clone());

                let view_name = format_ident!("{}", view_name);
                let view_type = format_ident!("K{}", view_type);
                let template_type = format_ident!("{}", consts::TEMPLATE_TYPE_NAME);

                let codegen_result = self.block().generate_code();
                match codegen_result {
                    Err(e) => {
                        result.add_error(e);
                        return result;
                    }
                    Ok(cgresult) => {
                        let imports_content = cgresult.imports;
                        let layout_content = cgresult.layout;
                        let render_content = cgresult.code;
                        let view_content = quote! {
                            use crate::viewtypes::*;
                            #imports_content

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
                                fn name() -> String {
                                    #full_view_name.to_string()
                                }

                                #layout_content

                                #render_content
                            }
                        };

                        _ = utils::fs::generate_code_with_content(&target, &view_content);
                    }
                }
            }
        }

        result
    }
}
