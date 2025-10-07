use crate::codegen::types::Block;
use crate::types::{error, result};
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn to_code_token_stream(
        &self,
    ) -> result::Result<TokenStream> {
        let code_span = match self {
            Block::KCODE(span) => span,
            _ => {
                return Err(error::CompileError::from_codegen(
                    &self,
                    "Wrong method call: couldn't generate code",
                ));
            }
        };

        if code_span.is_simple() {
            let raw_content = code_span.content();
            match raw_content.parse::<TokenStream>() {
                Ok(ts) => Ok(quote! {
                    #ts
                }),
                Err(err) => Err(error::CompileError::from_lex(&self, err)),
            }
        } else {
            let mut result = vec![];
            for block in code_span.blocks() {
                for ts in block.to_token_stream()? {
                    result.push(ts);
                }
            }
            Ok(quote! {
                #(#result)*
            })
        }
    }

    pub(in crate::codegen::compiler::types) fn to_inline_code_token_stream(
        &self,
    ) -> result::Result<TokenStream> {
        let code_span = match self {
            Block::KINLINEDCODE(span) => span,
            _ => {
                return Err(error::CompileError::from_codegen(
                    &self,
                    "Wrong method call: couldn't generate code",
                ));
            }
        };
        if code_span.is_simple() {
            let raw_content = code_span.content();
            match raw_content.parse::<TokenStream>() {
                Ok(ts) => Ok(quote! {
                    writer.write(&#ts.to_string());
                }),
                Err(err) => Err(error::CompileError::from_lex(&self, err)),
            }
        } else {
            Err(error::CompileError::from_codegen(
                &self,
                "Inlined content with nested blocks is not supported",
            ))
        }
    }
}
