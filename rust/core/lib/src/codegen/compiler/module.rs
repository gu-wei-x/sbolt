use crate::{
    codegen::{
        CompileResult, CompilerOptions,
        compiler::{fsutil, name},
        consts,
    },
    types::result,
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
    ) -> result::Result<CompileResult> {
        if !self.source.is_dir() {
            return Err(format!(
                "Source directory '{}' does not exist or is not a directory",
                self.source.display()
            )
            .into());
        }

        let dir_name = fsutil::get_dir_name(&self.source).ok_or(format!(
            "Failed to get directory name from {}",
            self.source.display()
        ))?;

        let target_dir = fsutil::create_target_dir(&self.target, &dir_name);
        let mut result = CompileResult::default();
        if let Ok(read_dir) = fs::read_dir(&self.source) {
            let name_space = name::create_name_space(&self.namespace, &dir_name);
            for entry in read_dir.flatten() {
                let entry = &entry;
                if let Ok(meta) = entry.metadata() {
                    if meta.is_dir() {
                        Module::new(entry.path(), target_dir.clone(), Some(name_space.clone()))
                            .process(compiler_options)?
                            .merge_into(&mut result);
                        result.add_mod(entry.file_name().to_str().unwrap_or_default());
                    } else if meta.is_file() {
                        let template_kind = fsutil::get_template_kind_from_ext(
                            &entry.path(),
                            compiler_options.extensions(),
                        );
                        if template_kind.is_none() {
                            // skip non-template files.
                            continue;
                        }

                        let content = fs::read_to_string(entry.path()).unwrap_or_default();
                        let template = match crate::codegen::types::Template::from(
                            &content,
                            Some(name_space.clone()),
                            template_kind.unwrap(),
                            compiler_options,
                        ) {
                            Ok(t) => t,
                            Err(e) => {
                                return Err(e.with_file(&entry.path()));
                            }
                        };

                        let file_name = fsutil::get_file_name(&entry.path()).unwrap_or_default();
                        let file_name = format!("{}{}", file_name, consts::RS_FILE_EXTENSION);
                        let target_file = target_dir.join(&file_name);
                        match template.compile(target_file, compiler_options) {
                            Ok(c_result) => {
                                c_result.merge_into(&mut result);
                            }
                            Err(e) => {
                                return Err(e.with_file(&entry.path()));
                            }
                        }

                        result.add_mod(&fsutil::get_file_name(&entry.path()).unwrap_or_default());
                    }
                }
            }

            // generate the mod.rs file.
            let ts = Self::generate_sub_mod_ts(result.mods())?;
            let mod_file = target_dir.join(consts::TEMPLATES_MOD_FILE_NAME);
            fsutil::write_code_to_file(&mod_file, &ts)?;
        }
        Ok(result)
    }
}

impl Module {
    fn generate_sub_mod_ts(mods: &[String]) -> result::Result<TokenStream> {
        let imported_content: String = mods
            .iter()
            .map(|m| format!("pub(crate) mod {};\n", m))
            .collect::<String>();
        let ts = imported_content
            .parse::<TokenStream>()
            .map_err(|err| format!("Failed to generate sub mod:{err}"))?;
        Ok(ts)
    }

    pub(crate) fn generate_root_mod_ts(
        mods: &[String],
        compiler_option: &CompilerOptions,
    ) -> TokenStream {
        let viewtypes_ident = format!(
            "{}::{}",
            consts::TEMPLATES_MAP_FILE_NAME,
            consts::TEMPLATES_TYPE_NAME
        );
        let mod_name = format_ident!("{}", compiler_option.mod_name());
        let import_content: String = mods
            .iter()
            .map(|m| format!("mod {};\n", m))
            .collect::<String>();
        let import_content_ts: TokenStream = import_content.parse().unwrap();
        let viewtypes_ident_ts: TokenStream = viewtypes_ident.parse().unwrap();
        quote! {
            #import_content_ts
            pub(crate) mod #mod_name {
                pub(crate) use super::*;

                // TemplateResolver.
                struct TemplateResolver {
                    view_creators: std::collections::HashMap<String, fn() -> #viewtypes_ident_ts>,
                }

                impl TemplateResolver {
                    fn new() -> Self {
                        Self {
                            view_creators: #viewtypes_ident_ts::create_view_registrar(),
                        }
                    }

                    fn resolve(&self, name: &str) -> Option<fn() -> #viewtypes_ident_ts> {
                        match sbolt::types::normalize_path_to_view_key(name) {
                            Some(key) => self.view_creators.get(&key).map(|f| *f),
                            None => None,
                        }
                    }
                }

                static TEMPLATE_RESOLVER: std::sync::LazyLock<TemplateResolver> = std::sync::LazyLock::new(|| {
                    TemplateResolver::new()
                });

                pub(crate) fn render(name: &str, context:&mut impl sbolt::types::Context) -> sbolt::types::result::RenderResult<String> {
                    if let Some(creator) = TEMPLATE_RESOLVER.resolve(name) {
                        let view = creator();
                        view.render(context)
                    } else {
                        Err(sbolt::types::error::RuntimeError::view_not_found(name))
                    }
                }

                #[allow(dead_code)]
                pub(crate) fn resolve_view_creator(name: &str) -> Option<fn() -> #viewtypes_ident_ts> {
                    TEMPLATE_RESOLVER.resolve(name)
                }
            }
        }
    }
}
