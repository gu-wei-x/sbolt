use crate::codegen::CompilerOptions;
use crate::codegen::types::Block;
use crate::types::error;
use crate::types::result;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn to_token_stream(
        &self,
        from: Option<&Block<'a>>,
    ) -> result::Result<Vec<TokenStream>> {
        let mut result = vec![];
        match self {
            Block::KCODE(_) => {
                let ts = self.to_code_token_stream(from)?;
                result.push(ts);
            }
            Block::KCOMMENT(_) => {
                // todo: output with option
                // ignore the comment block
            }
            Block::KCONTENT(_) => {
                let ts = self.to_content_token_stream()?;
                result.push(ts);
            }
            Block::KFUNCTIONS(_) => todo!(),
            Block::KINLINEDCODE(_) => {
                let ts = self.to_inline_code_token_stream()?;
                result.push(ts);
            }
            Block::KINLINEDCONTENT(_) => {
                let ts = self.to_inline_content_token_stream()?;
                result.push(ts);
            }
            Block::KROOT(span) => {
                // filter out layout and use was called before this.
                if span.blocks().is_empty() {
                    // todo: write the pure content
                    return Err(error::CompileError::from_codegen(
                        &self,
                        "Wrong method call: couldn't generate code",
                    ));
                } else {
                    for block in span.blocks() {
                        if !matches!(block, Block::KLAYOUT(_) | Block::KUSE(_)) {
                            for rs in block.to_token_stream(from)? {
                                result.push(rs);
                            }
                        }
                    }
                }
            }
            Block::KRENDER(_) => {
                let ts = self.to_render_token_stream()?;
                result.push(ts);
            }
            Block::KSECTION(_, _) => {
                let ts = self.to_section_token_stream()?;
                result.push(ts);
            }
            Block::KUSE(_) => {
                let use_ts = self.to_use_token_stream()?;
                result.push(use_ts);
            }
            Block::KLAYOUT(_) => {
                return Err(error::CompileError::from_codegen(
                    &self,
                    "Wrong method call: couldn't generate code",
                ));
            }
        }

        Ok(result)
    }

    pub(in crate::codegen::compiler::types) fn generate_render_token_stream(
        &self,
        compiler_options: &CompilerOptions,
    ) -> result::Result<TokenStream> {
        if !matches!(self, Block::KROOT(_)) {
            return Err(error::CompileError::from_codegen(
                &self,
                "Wrong method call: couldn't generate code",
            ));
        }

        let ts = self.to_token_stream(Some(self))?;
        let root_span = self.span();
        let has_layout = root_span
            .blocks()
            .iter()
            .any(|b| matches!(b, Block::KLAYOUT(_)));
        match has_layout {
            true => {
                let view_root_mod_name = format_ident!("{}", compiler_options.mod_name());
                let code = quote! {
                    fn render(&self, context:&mut impl sbolt::types::Context) -> sbolt::types::result::RenderResult<String> {
                        let mut writer = self.create_writer(None);
                        #(#ts)*
                        match Self::layout() {
                            Some(layout) => {
                                for key in sbolt::types::resolve_layout_to_view_keys(&layout, &Self::name()) {
                                    if let Some(creator) = crate::#view_root_mod_name::resolve_view_creator(&key) {
                                        context.set_default_section(writer.into_string());
                                        let view = creator();
                                        return view.render(context);
                                    }
                                }
                                Err(sbolt::types::error::RuntimeError::layout_not_found(&layout, &Self::name()))
                            }
                            None => Ok(writer.into_string()),
                        }
                    }
                };
                Ok(code)
            }
            false => {
                let code = quote! {
                    fn render(&self, #[allow(unused_variables)]context:&mut impl sbolt::types::Context) -> sbolt::types::result::RenderResult<String> {
                        let mut writer = self.create_writer(None);
                        // TODO: add other logic here
                        #(#ts)*
                        Ok(writer.into_string())
                    }
                };
                Ok(code)
            }
        }
    }
}
