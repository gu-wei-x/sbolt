use crate::{
    codegen::{CompileResult, consts, parser::template::Template},
    types::result,
    utils,
};
use quote::format_ident;
use quote::quote;
use std::path::PathBuf;

impl<'a> Template<'a> {
    pub(crate) fn compile(&self, target: PathBuf) -> result::Result<CompileResult> {
        match utils::fs::get_file_name(&target) {
            None => {
                return Err(format!("Failed to read template file: {}", target.display()).into());
            }
            Some(name) => {
                let mut result = CompileResult::default();
                let namespace = self.namespace().cloned();
                let full_view_name = utils::name::create_full_name(&namespace, &name);
                let view_name = utils::name::normalize_to_type_name(&name);
                let view_type = utils::name::normalize_to_type_name(&full_view_name);
                result.add_view_mapping(full_view_name.to_string(), view_name.clone());

                let view_name = format_ident!("{}", view_name);
                let view_type = format_ident!("K{}", view_type);
                let template_type = format_ident!("{}", consts::TEMPLATE_TYPE_NAME);

                let cgresult = self.block().generate_code()?;
                let imports_content = cgresult.imports;
                let layout_content = cgresult.layout;
                let render_content = cgresult.code;
                let view_content = quote! {
                    use crate::viewtypes::*;
                    use disguise::types::Context;

                    #imports_content

                    pub struct #view_name {
                        context: disguise::types::DefaultViewContext,
                    }

                    impl #view_name {
                        pub(crate) fn new(context: disguise::types::DefaultViewContext) -> Self {
                            Self {
                                context: context,
                            }
                        }

                        pub(crate) fn create(context: disguise::types::DefaultViewContext) -> #template_type {
                           #template_type::#view_type(#view_name::new(context))
                        }
                    }

                    impl disguise::types::Template for #view_name
                    {
                        fn name() -> String {
                            #full_view_name.to_string()
                        }

                        fn get_data<D: Send + Sync + 'static>(&self, key: &str) -> Option<&D> {
                            self.context.get_data(key)
                        }

                        #layout_content

                        #render_content
                    }
                };

                utils::fs::generate_code_with_content(&target, &view_content)?;
                Ok(result)
            }
        }
    }
}
