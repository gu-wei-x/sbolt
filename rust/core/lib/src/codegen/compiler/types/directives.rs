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

    pub(in crate::codegen::compiler::types) fn generate_imports(
        &self,
    ) -> result::Result<Vec<TokenStream>> {
        let root_span = match self {
            Block::KROOT(span) => span,
            _ => {
                return Err(error::CompileError::from_codegen(
                    &self,
                    "Wrong method call: couldn't generate code",
                ));
            }
        };

        let use_statements = root_span
            .blocks()
            .iter()
            .filter(|b| matches!(b, Block::KUSE(_)))
            .map(|b| format!("{} {};", consts::DIRECTIVE_KEYWORD_USE, b.content()))
            .collect::<Vec<_>>();
        let mut use_ts = vec![];
        for statement in use_statements {
            let statement_ts = statement.parse::<TokenStream>();
            match statement_ts {
                Ok(ts) => use_ts.push(ts),
                Err(err) => return Err(error::CompileError::from_lex(self, err)),
            }
        }

        Ok(use_ts)
    }

    // template has default implementation: option.
    pub(in crate::codegen::compiler::types) fn generate_layout(
        &self,
    ) -> result::Result<Option<TokenStream>> {
        let root_span = match self {
            Block::KROOT(span) => span,
            _ => {
                return Err(error::CompileError::from_codegen(
                    &self,
                    "Wrong method call: couldn't generate code",
                ));
            }
        };

        let layout_blocks = root_span
            .blocks()
            .iter()
            .filter(|b| matches!(b, Block::KLAYOUT(_)))
            .collect::<Vec<_>>();
        match layout_blocks.len() {
            usize::MIN..=0 => Ok(None),
            1 => {
                let content = layout_blocks[0].content();
                Ok(Some(quote! {
                    fn layout() -> Option<String> {
                        Some(#content.to_string())
                    }
                }))
            }
            _ => Err(error::CompileError::from_codegen(
                self,
                "Multiple layout directives found",
            )),
        }
    }
}
