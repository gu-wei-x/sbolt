use crate::codegen::compiler::context::CodeGenContext;
use crate::codegen::types::Block;
use crate::types::{error, result};
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    fn static_output(&self) -> Option<(String, usize)> {
        match self {
            Block::KCOMMENT(_) => Some((String::new(), 0)),
            Block::KCONTENT(span) | Block::KINLINEDCONTENT(span) if span.is_simple() => {
                Some((span.content(), 1))
            }
            Block::KCONTENT(span) => {
                let mut content = String::new();
                let mut writes = 0;
                for block in span.blocks() {
                    let (part, count) = block.static_output()?;
                    content.push_str(&part);
                    writes += count;
                }
                Some((content, writes))
            }
            _ => None,
        }
    }

    pub(in crate::codegen::compiler::types) fn content_blocks_to_token_stream<'b>(
        blocks: impl IntoIterator<Item = &'b Block<'a>>,
        from: Option<&Block<'a>>,
        context: &CodeGenContext,
    ) -> result::Result<Vec<TokenStream>>
    where
        'a: 'b,
    {
        let mut result = Vec::new();
        let mut pending = String::new();
        let mut writes = 0;
        for block in blocks {
            if context.optimize_static_output() {
                if let Some((content, count)) = block.static_output() {
                    pending.push_str(&content);
                    writes += count;
                    continue;
                }
                if writes > 0 {
                    result.push(Self::static_write(&pending, writes, context));
                    pending.clear();
                    writes = 0;
                }
            }
            result.extend(block.to_token_stream(from, context)?);
        }
        if writes > 0 {
            result.push(Self::static_write(&pending, writes, context));
        }
        Ok(result)
    }

    fn static_write(content: &str, writes: usize, context: &CodeGenContext) -> TokenStream {
        match context.static_content(content, writes) {
            Some(content) => quote! { writer.write(#content); },
            None => TokenStream::new(),
        }
    }

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
            Ok(Self::static_write(&content_span.content(), 1, context))
        } else {
            let result =
                Self::content_blocks_to_token_stream(content_span.blocks(), Some(self), context)?;
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
            Ok(Self::static_write(&content_span.content(), 1, context))
        } else {
            Err(error::CompileError::from_codegen(
                &self,
                "Inlined content with nested blocks is not supported",
            ))
        }
    }
}
