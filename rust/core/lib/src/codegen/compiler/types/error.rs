use proc_macro2::LexError;

use crate::codegen::types::Block;
use crate::types::error;

impl error::CompileError {
    pub(in crate::codegen::compiler) fn from_lex(block: &Block<'_>, error: LexError) -> Self {
        error::CompileError::CodeGen(
            block.location(),
            error.to_string(),
            block.content().to_string(),
        )
    }

    pub(in crate::codegen::compiler) fn from_codegen(block: &Block<'_>, str: &str) -> Self {
        error::CompileError::CodeGen(
            block.location(),
            str.to_string(),
            block.content().to_string(),
        )
    }
}
