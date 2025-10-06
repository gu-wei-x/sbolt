#![allow(dead_code)]
use crate::codegen::types::Block;
use crate::types::result;
use proc_macro2::TokenStream;

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
            Block::KROOT(_span) => todo!(),
            Block::KRENDER(_span) => todo!(),
            Block::KSECTION(_, _span) => todo!(),
            Block::KUSE(_span) => Self::generate_use(self),
        }
    }
}
