#![allow(dead_code)]
use crate::codegen::types::Block;
use crate::types::error;
use crate::types::result;
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn generate_code(&self) -> result::Result<TokenStream> {
        match self {
            Block::KCODE(_span) => todo!(),
            Block::KCOMMENT(_span) => todo!(),
            Block::KCONTENT(_span) => todo!(),
            Block::KFUNCTIONS(_span) => todo!(),
            Block::KINLINEDCODE(_span) => todo!(),
            Block::KINLINEDCONTENT(_span) => todo!(),
            Block::KLAYOUT(_span) => Self::generate_layout(self),
            Block::KROOT(_span) => Self::generate_root(&self),
            Block::KRENDER(_span) => todo!(),
            Block::KSECTION(_, _span) => todo!(),
            Block::KUSE(_span) => Self::generate_use(self),
        }
    }

    pub(in crate::codegen::compiler::types) fn generate_root(&self) -> result::Result<TokenStream> {
        if !matches!(self, Block::KROOT(_)) {
            return Err(error::CompileError::from_codegen(
                &self,
                "Wrong method call: couldn't generate code",
            ));
        }

        let code = quote! {
            fn render(&self) -> disguise::types::result::RenderResult<String> {
                let mut writer = disguise::types::HtmlWriter::new();
                // TODO: add other logic here
                match Self::layout() {
                    Some(layout) => {
                        for key in disguise::types::resolve_layout_to_view_keys(&layout, &Self::name()) {
                            if let Some(creator) = crate::basic_views::resolve_view_creator(&key) {
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
