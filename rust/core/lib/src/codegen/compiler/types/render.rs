use crate::codegen::types::Block;
use crate::types::{error, result};
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn to_render_token_stream(
        &self,
    ) -> result::Result<TokenStream> {
        let content_span = match self {
            Block::KRENDER(span) => span,
            _ => {
                return Err(error::CompileError::from_codegen(
                    &self,
                    "Wrong method call: couldn't generate code",
                ));
            }
        };
        match content_span.blocks().len() {
            0 => {
                let ts = quote! {
                    // No parameters in @render()
                   let default_section = context.get_default_section();
                   match default_section {
                       Some(content) => writer.write(&content),
                       None => {
                           return Err(disguise::types::error::RuntimeError::NotFound("Default section not found".to_string(), "".to_string()))
                       }
                   }
                };
                return Ok(ts);
            }
            1 => {
                // todo: envaluated dynamically.
                let section_name = content_span.blocks()[0].content();
                let ts = quote! {
                    // 1 parameters in @render()
                   let section_name = #section_name;
                   let sections = context.get_section(section_name);
                   match sections {
                       Some(contents) => {
                        for content in contents {
                            writer.write(&content)
                        }
                       },
                       None => {
                           return Err(disguise::types::error::RuntimeError::NotFound("Default section not found".to_string(), "".to_string()))
                       }
                   }
                };
                return Ok(ts);
            }
            2 => {
                // todo: envaluated dynamically.
                let section_name = content_span.blocks()[0].content();
                let is_required = content_span.blocks()[1].content(); // let is_bool = text.parse::<bool>().is_ok();
                let ts = quote! {
                    // 2 parameters in @render()
                   let section_name = #section_name;
                   let is_required = #is_required.parse::<bool>().is_ok();
                   let sections = context.get_section(section_name);
                   match sections {
                       Some(contents) => {
                        for content in contents {
                            writer.write(&content)
                        }
                       },
                       None if is_required => {
                           return Err(disguise::types::error::RuntimeError::NotFound("Default section not found".to_string(), "".to_string()))
                       }
                       _ => {
                          /*ignore */
                       }
                   }
                };
                return Ok(ts);
            }
            _ => Err(error::CompileError::from_codegen(
                &self,
                "Wrong number of parameters in @render(), expected 0, 1 or 2",
            )),
        }
    }
}
