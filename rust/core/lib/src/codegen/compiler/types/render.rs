#![allow(dead_code)]
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

        // todo: implement.
        let raw_content = content_span.content();
        let ts = quote! {
            writer.write(#raw_content);
        };
        Ok(ts)
    }
}
