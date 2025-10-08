use crate::codegen::types::Block;
use crate::types::{error, result};
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn to_section_token_stream(
        &self,
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

        let ts = match span.is_simple() {
            true => {
                // simple is content section.
                let raw_content = span.content();
                quote! {
                    let _name = #name;
                    let _inner_writer = {
                        let mut writer = disguise::types::HtmlWriter::new();
                        writer.write(#raw_content);
                        writer
                    };
                    // todo: add to sections.
                }
            }
            false => {
                let mut tsv = vec![];
                for block in span.blocks() {
                    for rs in block.to_token_stream()? {
                        tsv.push(rs);
                    }
                }
                quote! {
                    let _name = #name;
                    let _inner_writer = {
                        let mut writer = disguise::types::HtmlWriter::new();
                        #(#tsv)*
                        writer
                    };
                    // todo: add to sections.
                }
            }
        };
        Ok(ts)
    }
}
