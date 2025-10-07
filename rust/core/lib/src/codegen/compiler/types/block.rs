#![allow(dead_code)]
use crate::codegen::types::Block;
use crate::types::error;
use crate::types::result;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn to_token_stream(
        &self,
    ) -> result::Result<Vec<TokenStream>> {
        let mut result = vec![];
        match self {
            Block::KCODE(_) => {
                let ts = self.to_code_token_stream()?;
                result.push(ts);
            }
            Block::KCOMMENT(_) => {
                // todo: output with option
                // ignore the comment block
            }
            Block::KCONTENT(_) => {
                let ts = self.to_content_token_stream()?;
                result.push(ts);
            }
            Block::KFUNCTIONS(_) => todo!(),
            Block::KINLINEDCODE(_) => {
                let ts = self.to_inline_code_token_stream()?;
                result.push(ts);
            }
            Block::KINLINEDCONTENT(_) => {
                let ts = self.to_inline_content_token_stream()?;
                result.push(ts);
            }
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
                            for rs in block.to_token_stream()? {
                                result.push(rs);
                            }
                        }
                    }
                }
            }
            Block::KRENDER(_) => {
                let ts = self.to_render_token_stream()?;
                result.push(ts);
            }
            Block::KSECTION(_, _) => {
                let ts = self.to_section_token_stream()?;
                result.push(ts);
            }
            Block::KUSE(_) => {
                let use_ts = self.to_use_token_stream()?;
                result.push(use_ts);
            }
            Block::KLAYOUT(_) => {
                return Err(error::CompileError::from_codegen(
                    &self,
                    "Wrong method call: couldn't generate code",
                ));
            }
        }

        Ok(result)
    }

    pub(in crate::codegen::compiler::types) fn generate_render_token_stream(
        &self,
        mod_name: &str,
    ) -> result::Result<TokenStream> {
        if !matches!(self, Block::KROOT(_)) {
            return Err(error::CompileError::from_codegen(
                &self,
                "Wrong method call: couldn't generate code",
            ));
        }

        let has_layout = match self {
            Block::KROOT(root_span) => root_span
                .blocks()
                .iter()
                .any(|b| matches!(b, Block::KLAYOUT(_))),
            _ => false,
        };
        let mut ts = self.to_token_stream()?;
        let view_root_mod_name = format_ident!("{}", mod_name);
        let layout_logic_ts = if has_layout {
            quote! {
                match Self::layout() {
                    Some(layout) => {
                        for key in disguise::types::resolve_layout_to_view_keys(&layout, &Self::name()) {
                            if let Some(creator) = crate::#view_root_mod_name::resolve_view_creator(&key) {
                                let view = creator(disguise::context!());
                                return view.render();
                            }
                        }
                        Err(disguise::types::error::RuntimeError::layout_not_found(&layout, &Self::name()))
                    }
                    None => Ok(writer.into_string()),
                }
            }
        } else {
            quote! {
                Ok(writer.into_string())
            }
        };
        ts.push(layout_logic_ts);
        let code = quote! {
            fn render(&self) -> disguise::types::result::RenderResult<String> {
                let mut writer = disguise::types::HtmlWriter::new();
                // TODO: add other logic here
                #(#ts)*
            }
        };
        Ok(code)
    }
}
