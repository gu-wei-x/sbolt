use crate::codegen::compiler::context::CodeGenContext;
use crate::codegen::types::Block;
use crate::types::{error, result};
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn to_code_token_stream(
        &self,
        from: Option<&Block<'a>>,
        context: &CodeGenContext,
    ) -> result::Result<TokenStream> {
        // validate parent block.
        from.ok_or(error::CompileError::from_codegen(
            &self,
            "Parent block is required to generate code",
        ))?;

        if !matches!(self, Block::KCODE(_)) {
            return Err(error::CompileError::from_codegen(
                &self,
                "Wrong method call: couldn't generate code",
            ));
        }
        let code_span = self.span();
        let mut code_content = String::new();
        if code_span.is_simple() {
            let raw_content = code_span.content();
            code_content.push_str(&raw_content);
        } else {
            for block in code_span.blocks() {
                if matches!(block, Block::KCODE(_)) {
                    let raw_content = block.content();
                    code_content.push_str(&raw_content);
                } else {
                    for ts in block.to_token_stream(from, context)? {
                        code_content.push_str(&ts.to_string());
                    }
                }
            }
        }

        match code_content.parse::<TokenStream>() {
            Ok(ts) => Ok(ts),
            Err(err) => Err(error::CompileError::from_lex(&self, err)),
        }
    }

    pub(in crate::codegen::compiler::types) fn to_inline_code_token_stream(
        &self,
    ) -> result::Result<TokenStream> {
        if !matches!(self, Block::KINLINEDCODE(_)) {
            return Err(error::CompileError::from_codegen(
                &self,
                "Wrong method call: couldn't generate code",
            ));
        }
        let code_span = self.span();
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
