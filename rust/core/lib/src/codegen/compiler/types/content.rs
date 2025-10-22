use crate::codegen::compiler::context::CodeGenContext;
use crate::codegen::types::Block;
use crate::types::{error, result};
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn to_content_token_stream(
        &self,
        context: &CodeGenContext,
    ) -> result::Result<TokenStream> {
        if !matches!(self, Block::KCONTENT(_)) {
            return Err(error::CompileError::from_codegen(
                &self,
                "Wrong method call: couldn't generate code",
            ));
        }

        let content_span = self.span();
        if content_span.is_simple() {
            let raw_content = content_span.content();
            let optimizer = context.create_optimizer(self);
            let raw_content = optimizer.optimize(&raw_content);
            let ts = quote! {
                writer.write(#raw_content);
            };
            Ok(ts)
        } else {
            let mut result = vec![];
            for block in content_span.blocks() {
                for ts in block.to_token_stream(Some(self), context)? {
                    result.push(ts);
                }
            }
            Ok(quote! {
                #(#result)*
            })
        }
    }

    pub(in crate::codegen::compiler::types) fn to_inline_content_token_stream(
        &self,
        context: &CodeGenContext,
    ) -> result::Result<TokenStream> {
        // check whether block is pure content or compitation block.
        if !matches!(self, Block::KINLINEDCONTENT(_)) {
            return Err(error::CompileError::from_codegen(
                &self,
                "Wrong method call: couldn't generate code",
            ));
        }

        let content_span = self.span();
        if content_span.is_simple() {
            let raw_content = content_span.content();
            let optimizer = context.create_optimizer(self);
            let raw_content = optimizer.optimize(&raw_content);
            let ts = quote! {
                writer.write(#raw_content);
            };
            Ok(ts)
        } else {
            Err(error::CompileError::from_codegen(
                &self,
                "Inlined content with nested blocks is not supported",
            ))
        }
    }
}
