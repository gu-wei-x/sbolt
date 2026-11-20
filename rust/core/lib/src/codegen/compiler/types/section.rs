use crate::codegen::compiler::context::CodeGenContext;
use crate::codegen::types::Block;
use crate::types::{error, result};
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn to_section_token_stream(
        &self,
        context: &mut CodeGenContext<'_>,
    ) -> result::Result<TokenStream> {
        let (name, span) = match self {
            Block::KSECTION(name, span) => (name, span),
            _ => {
                return Err(error::CompileError::from_codegen(
                    &self,
                    "Wrong method call: couldn't generate code",
                ));
            }
        };
        let mut section_context = context.section_context();
        let context = &mut section_context;

        let ts = match span.is_simple() {
            true => {
                // simple is content section.
                let content = Block::new_content(span.clone());
                let write = content.to_content_token_stream(context)?;
                let capacity_hint = context.capacity_hint();
                quote! {
                    let section_name = #name;
                    let inner_writer = {
                        let mut writer = self.create_writer(None, Some(#capacity_hint));
                        #write
                        writer
                    };
                    context.add_section(section_name, inner_writer.into_string());
                }
            }
            false => {
                let tsv = Self::content_blocks_to_token_stream(span.blocks(), Some(self), context)?;
                let capacity_hint = context.capacity_hint();
                quote! {
                    let section_name = #name;
                    let section_writer = {
                        let mut writer = self.create_writer(None, Some(#capacity_hint));
                        #(#tsv)*
                        writer
                    };
                    context.add_section(section_name, section_writer.into_string());
                }
            }
        };
        Ok(ts)
    }
}
