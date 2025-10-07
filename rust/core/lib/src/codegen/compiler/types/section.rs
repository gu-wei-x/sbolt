#![allow(dead_code)]
use crate::codegen::types::Block;
use crate::types::{error, result};
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn to_section_token_stream(
        &self,
    ) -> result::Result<TokenStream> {
        let (_name, _span) = match self {
            Block::KSECTION(name, span) => (name, span),
            _ => {
                return Err(error::CompileError::from_codegen(
                    &self,
                    "Wrong method call: couldn't generate code",
                ));
            }
        };

        // todo: implement.
        let ts = quote! {
            writer.write("todo: implement section");
        };
        Ok(ts)
    }
}
