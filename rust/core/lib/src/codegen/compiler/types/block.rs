#![allow(dead_code)]
use crate::codegen::types::Block;
use crate::types::error;
use crate::types::result;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn generate_code(
        &self,
    ) -> result::Result<Vec<TokenStream>> {
        let mut result = vec![];
        match self {
            Block::KCODE(_span) => todo!(),
            Block::KCOMMENT(_span) => todo!(),
            Block::KCONTENT(_span) => todo!(),
            Block::KFUNCTIONS(_span) => todo!(),
            Block::KINLINEDCODE(_span) => todo!(),
            Block::KINLINEDCONTENT(_span) => todo!(),
            Block::KROOT(span) => {
                // filter out layout and use was called before this.
                if span.blocks().is_empty() {
                    // todo: write the pure content
                    return Err(error::CompileError::from_codegen(
                        &self,
                        "Wrong method call: couldn't generate code",
                    ));
                } else {
                    for block in span.blocks() {
                        if !matches!(block, Block::KLAYOUT(_) | Block::KUSE(_)) {
                            for rs in block.generate_code()? {
                                result.push(rs);
                            }
                        }
                    }
                }
            }
            Block::KRENDER(_span) => todo!(),
            Block::KSECTION(_, _span) => todo!(),
            Block::KUSE(_span) => {
                let use_ts = self.generate_use()?;
                result.push(use_ts);
            }
            Block::KLAYOUT(_span) => {
                return Err(error::CompileError::from_codegen(
                    &self,
                    "Wrong method call: couldn't generate code",
                ));
            }
        }

        Ok(result)
    }

    pub(in crate::codegen::compiler::types) fn generate_render(
        &self,
        mod_name: &str,
    ) -> result::Result<TokenStream> {
        if !matches!(self, Block::KROOT(_)) {
            return Err(error::CompileError::from_codegen(
                &self,
                "Wrong method call: couldn't generate code",
            ));
        }

        let mod_name_id = format_ident!("{}", mod_name);
        let code = quote! {
            fn render(&self) -> disguise::types::result::RenderResult<String> {
                let mut writer = disguise::types::HtmlWriter::new();
                // TODO: add other logic here
                match Self::layout() {
                    Some(layout) => {
                        for key in disguise::types::resolve_layout_to_view_keys(&layout, &Self::name()) {
                            if let Some(creator) = crate::#mod_name_id::resolve_view_creator(&key) {
                                let view = creator(disguise::context!());
                                return view.render();
                            }
                        }
                        Err(disguise::types::error::RuntimeError::layout_not_found(&layout, &Self::name()))
                    }
                    None => Ok(writer.into_string()),
                }
            }
        };
        Ok(code)
    }
}
