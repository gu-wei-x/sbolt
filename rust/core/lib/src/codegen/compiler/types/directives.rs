#![allow(dead_code)]
use crate::codegen::types::Block;
use proc_macro2::TokenStream;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn generate_use(&self) -> Option<TokenStream> {
        None
    }
}
