use crate::{
    codegen::{
        CompileResult,
        compiler::{fsutil, name},
        consts,
        parser::template::Template,
    },
    types::result,
};
use quote::format_ident;
use quote::quote;
use std::path::PathBuf;

impl<'a> Template<'a> {
    pub(crate) fn compile(&self, target: PathBuf) -> result::Result<CompileResult> {
        match fsutil::get_file_name(&target) {
            None => {
                return Err(format!("Failed to read template file: {}", target.display()).into());
            }
            Some(name) => {
                let mut result = CompileResult::default();
                let namespace = self.namespace().cloned();

                let namespace = name::create_name_space(&namespace, &name);
                let view_name = name::create_view_type_name(&name);
                let full_view_name = name::create_normalized_name(&Some(namespace), &view_name);
                let view_type = name::create_view_type_name(&full_view_name);
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
                    use disguise::types::Writer;

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

                println!("******************************{}", view_content.to_string());
                fsutil::write_code_to_file(&target, &view_content)?;
                Ok(result)
            }
        }
    }
}
