#![allow(dead_code)]
use crate::codegen::{consts, types::Block};
use crate::types::{error, result};
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn generate_use(&self) -> result::Result<TokenStream> {
        match self {
            Block::KUSE(span) => {
                let statement = format!("{} {};", consts::DIRECTIVE_KEYWORD_USE, span.content());
                let result = statement.parse::<TokenStream>();
                match result {
                    Ok(ts) => Ok(ts),
                    Err(err) => Err(error::CompileError::from_lex(&self, err)),
                }
            }
            _ => Err(error::CompileError::from_codegen(
                &self,
                "Wrong method call: couldn't generate code",
            )),
        }
    }

    pub(in crate::codegen::compiler::types) fn generate_layout(
        &self,
    ) -> result::Result<TokenStream> {
        match self {
            Block::KLAYOUT(span) => {
                let content = span.content();
                Ok(quote! {
                    fn layout() -> Option<String> {
                        Some(#content.to_string())
                    }
                })
            }
            _ => Err(error::CompileError::from_codegen(
                &self,
                "Wrong method call: couldn't generate code",
            )),
        }
    }
}
