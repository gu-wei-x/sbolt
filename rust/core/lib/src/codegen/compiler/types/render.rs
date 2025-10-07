use crate::codegen::types::Block;
use crate::types::{error, result};
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn to_render_token_stream(
        &self,
    ) -> result::Result<TokenStream> {
        let content_span = match self {
            Block::KRENDER(span) => span,
            _ => {
                return Err(error::CompileError::from_codegen(
                    &self,
                    "Wrong method call: couldn't generate code",
                ));
            }
        };
        match content_span.blocks().len() {
            0 => {
                let ts = quote! {
                    // No parameters in @render()
                   writer.write("todo: implement render default here");
                };
                return Ok(ts);
            }
            1 => {
                let ts = quote! {
                    // No parameters in @render()
                   writer.write("todo: implement render 1 here");
                };
                return Ok(ts);
            }
            2 => {
                let ts = quote! {
                    // No parameters in @render()
                   writer.write("todo: implement render 2 here");
                };
                return Ok(ts);
            }
            _ => Err(error::CompileError::from_codegen(
                &self,
                "Wrong number of parameters in @render(), expected 0, 1 or 2",
            )),
        }
    }
}
