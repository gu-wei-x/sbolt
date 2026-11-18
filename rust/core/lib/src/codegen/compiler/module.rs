use crate::{
    codegen::{
        CompileResult, CompilerOptions,
        compiler::{fsutil, name},
        consts,
    },
    types::{result, template::Kind},
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
                            .merge_into(&mut result)?;
                        result.add_mod(entry.file_name().to_str().unwrap_or_default());
                    } else if meta.is_file() {
                        let Some(template_kind) = fsutil::get_template_kind_from_ext(
                            &entry.path(),
                            compiler_options.extensions(),
                        ) else {
                            // skip non-template files.
                            continue;
                        };

                        let content = fs::read_to_string(entry.path()).unwrap_or_default();
                        let template = match crate::codegen::types::Template::from(
                            &content,
                            Some(name_space.clone()),
                            template_kind,
                            compiler_options,
                        ) {
                            Ok(t) => t,
                            Err(e) => {
                                return Err(e.with_file(&entry.path()));
                            }
                        };

                        let source_name = fsutil::get_file_name(&entry.path()).unwrap_or_default();
                        let suffix = match template_kind {
                            Kind::KHTML => "html",
                            Kind::KJSON => "json",
                            Kind::KTEXT => "text",
                        };
                        let mut module_name = format!("{source_name}_{suffix}");
                        while self.source.join(&module_name).is_dir() {
                            module_name.push('_');
                        }
                        let file_name = format!("{}{}", module_name, consts::RS_FILE_EXTENSION);
                        let target_file = target_dir.join(&file_name);
                        match template.compile(&source_name, target_file, compiler_options) {
                            Ok(c_result) => {
                                c_result.merge_into(&mut result)?;
                            }
                            Err(e) => {
                                return Err(e.with_file(&entry.path()));
                            }
                        }

                        result.add_mod(&module_name);
                    }
                }
            }

            // generate the mod.rs file.
            let ts = self.generate_sub_mod_ts(result.mods())?;
            let mod_file = target_dir.join(consts::TEMPLATES_MOD_FILE_NAME);
            fsutil::write_code_to_file(&mod_file, &ts)?;
        }
        Ok(result)
    }
}

impl Module {
    fn generate_sub_mod_ts(&self, mods: &[String]) -> result::Result<TokenStream> {
        let mut declarations = Vec::new();
        for module in mods {
            let ident = syn::parse_str::<syn::Ident>(module)?;
            let path = if self.source.join(module).is_dir() {
                PathBuf::from(module).join(consts::TEMPLATES_MOD_FILE_NAME)
            } else {
                PathBuf::from(format!("{module}{}", consts::RS_FILE_EXTENSION))
            };
            let path = path.to_str().ok_or("Failed to create a module file path")?;
            declarations.push(quote! {
                #[path = #path]
                pub(crate) mod #ident;
            });
        }
        Ok(quote! { #(#declarations)* })
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
                    view_creators: std::collections::HashMap<String, std::collections::HashMap<sbolt::types::template::Kind, fn() -> #viewtypes_ident_ts>>,
                }

                impl TemplateResolver {
                    fn new() -> Self {
                        Self {
                            view_creators: #viewtypes_ident_ts::create_view_registrar(),
                        }
                    }

                    fn resolve_as(&self, name: &str, kind: sbolt::types::template::Kind) -> Option<fn() -> #viewtypes_ident_ts> {
                        let key = sbolt::types::normalize_path_to_view_key(name)?;
                        self.view_creators.get(&key)?.get(&kind).copied()
                    }
                }

                static TEMPLATE_RESOLVER: std::sync::LazyLock<TemplateResolver> = std::sync::LazyLock::new(|| {
                    TemplateResolver::new()
                });

                #[allow(dead_code)]
                pub(crate) fn render(
                    name: &str,
                    kind: Option<sbolt::types::template::Kind>,
                    context: &mut impl sbolt::types::Context,
                ) -> sbolt::types::result::RenderResult<(String, sbolt::types::template::Kind)> {
                    let kind = kind.unwrap_or(sbolt::types::template::Kind::KHTML);
                    let creator = TEMPLATE_RESOLVER.resolve_as(name, kind)
                        .ok_or_else(|| sbolt::types::error::RuntimeError::view_kind_not_found(name, kind))?;
                    creator().render(context).map(|output| (output, kind))
                }

                #[allow(dead_code)]
                pub(crate) fn resolve_view_creator(name: &str, kind: sbolt::types::template::Kind) -> Option<fn() -> #viewtypes_ident_ts> {
                    TEMPLATE_RESOLVER.resolve_as(name, kind)
                }
            }
        }
    }
}
