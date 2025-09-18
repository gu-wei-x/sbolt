use crate::{
    codegen::{CompileResult, consts},
    types::error,
    utils,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{fs, path::PathBuf};

pub(crate) struct Module {
    source: PathBuf,
    target: PathBuf,
    namespace: Option<String>,
}

impl Module {
    pub(crate) fn new(source: PathBuf, target: PathBuf, namespace: Option<String>) -> Self {
        Self {
            source: source,
            target: target,
            namespace,
        }
    }

    pub(crate) fn process(
        &self,
        compiler_options: &crate::codegen::compiler::CompilerOptions,
    ) -> CompileResult {
        let mut result = CompileResult::default();
        if !self.source.is_dir() {
            result.add_error(error::Error::from_str(&format!(
                "Source directory '{}' does not exist or is not a directory",
                self.source.display()
            )));
            return result;
        }

        let dir_name = utils::fs::get_dir_name(&self.source).unwrap();
        let target_dir = utils::fs::create_target_dir(&self.target, &dir_name);
        if let Ok(read_dir) = fs::read_dir(&self.source) {
            let full_name = &Some(utils::name::create_full_name(&self.namespace, &dir_name));
            for entry in read_dir.flatten() {
                let entry = &entry;
                if let Ok(meta) = entry.metadata() {
                    if meta.is_dir() {
                        Module::new(entry.path(), target_dir.clone(), full_name.clone())
                            .process(compiler_options)
                            .merge_into(&mut result);
                        result.add_mod(entry.file_name().to_str().unwrap_or_default());
                    } else if meta.is_file()
                        && utils::fs::match_file_with_ext(
                            &entry.path(),
                            &compiler_options.extensions,
                        )
                    {
                        let content = fs::read_to_string(entry.path()).unwrap_or_default();
                        let template = crate::codegen::parser::template::Template::from(
                            &content,
                            full_name.clone(),
                        )
                        .unwrap();

                        let file_name = utils::fs::get_file_name(&entry.path()).unwrap_or_default();
                        let file_name = format!("{}{}", file_name, consts::RS_FILE_EXTENSION);
                        let target_file = target_dir.join(&file_name);
                        template.compile(target_file).merge_into(&mut result);
                        result
                            .add_mod(&utils::fs::get_file_name(&entry.path()).unwrap_or_default());
                    }
                }
            }

            // generate the mod.rs file.
            let ts = Self::generate_sub_mod_ts(result.mods());
            let mod_file = target_dir.join(consts::TEMPLATES_MOD_FILE_NAME);
            _ = utils::fs::generate_code_with_content(&mod_file, &ts);
        }

        result
    }
}

impl Module {
    fn generate_sub_mod_ts(mods: &[String]) -> TokenStream {
        let imported_content: String = mods
            .iter()
            .map(|m| format!("pub(crate) mod {};\n", m))
            .collect::<String>();
        imported_content.parse().unwrap()
    }

    pub(crate) fn generate_root_mod_ts(mod_name: &str, mods: &[String]) -> TokenStream {
        let viewtypes_ident = format!(
            "{}::{}",
            consts::TEMPLATES_MAP_FILE_NAME,
            consts::TEMPLATE_TYPE_NAME
        );

        let mod_name = format_ident!("{}", mod_name);
        let import_content: String = mods
            .iter()
            .map(|m| format!("mod {};\n", m))
            .collect::<String>();
        let re_export_content: String = mods
            .iter()
            .map(|m| {
                format!(
                    "pub(crate) mod {} {{ pub(crate) use super::super::{}::*; }}\n",
                    m, m
                )
            })
            .collect::<String>();

        let import_content_ts: TokenStream = import_content.parse().unwrap();
        let re_export_content_ts: TokenStream = re_export_content.parse().unwrap();
        let viewtypes_ident_ts: TokenStream = viewtypes_ident.parse().unwrap();

        quote! {
            #import_content_ts
            pub mod #mod_name {
                #re_export_content_ts

                // TemplateResolver.
                struct TemplateResolver {
                    view_creators: std::collections::HashMap<&'static str, fn() -> #viewtypes_ident_ts>,
                }

                impl TemplateResolver {
                    fn new() -> Self {
                        Self {
                            view_creators: #viewtypes_ident_ts::create_view_registrar(),
                        }
                    }

                    fn resolve(&self, name: &str) -> Option<#viewtypes_ident_ts> {
                        let normalized_name: &str = &disguise::utils::name::normalize_name(name);
                        self.view_creators.get(normalized_name).map(|f| f())
                    }
                }

                static TEMPLATE_RESOLVER: std::sync::LazyLock<TemplateResolver> = std::sync::LazyLock::new(|| {
                    TemplateResolver::new()
                });

                pub(crate) fn render(name: &str, context: &mut impl disguise::types::Context) {
                     if let Some(view) = TEMPLATE_RESOLVER.resolve(name) {
                        view.render(context);
                     }
                }
            }
        }
    }
}
