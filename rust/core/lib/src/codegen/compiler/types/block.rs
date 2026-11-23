use crate::codegen::compiler::context::CodeGenContext;
use crate::codegen::types::{Block, ModelDeclaration};
use crate::types::error;
use crate::types::result;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn to_token_stream(
        &self,
        from: Option<&Block<'a>>,
        context: &mut CodeGenContext<'_>,
    ) -> result::Result<Vec<TokenStream>> {
        let mut result = vec![];
        match self {
            Block::KCODE(_) => {
                let ts = self.to_code_token_stream(from, context)?;
                result.push(ts);
            }
            Block::KCOMMENT(_) => {
                // todo: output with option
                // ignore the comment block
            }
            Block::KCONTENT(_) => {
                let ts = self.to_content_token_stream(context)?;
                result.push(ts);
            }
            Block::KFUNCTIONS(_) => todo!(),
            Block::KINLINEDCODE(_) => {
                context.invalidate_static_context();
                let ts = self.to_inline_code_token_stream(context)?;
                result.push(ts);
            }
            Block::KINLINEDCONTENT(_) => {
                let ts = self.to_inline_content_token_stream(context)?;
                result.push(ts);
            }
            Block::KROOT(span) => {
                let fully_static = span.blocks().iter().all(Self::is_static_output);
                context.prepare_root(fully_static);
                // filter out layout and use was called before this.
                if span.blocks().is_empty() {
                    // todo: write the pure content
                    return Err(error::CompileError::from_codegen(
                        &self,
                        "Wrong method call: couldn't generate code",
                    ));
                } else {
                    let blocks = span.blocks().iter().filter(|b| {
                        !matches!(b, Block::KLAYOUT(_) | Block::KMODEL(_) | Block::KUSE(_))
                    });
                    let tokens = Self::content_blocks_to_token_stream(blocks, from, context)
                        .map_err(|error| {
                            if fully_static {
                                self.remap_static_error(error)
                            } else {
                                error
                            }
                        })?;
                    result.extend(tokens);
                }
            }
            Block::KRENDER(_) => {
                context.invalidate_static_context();
                let ts = self.to_render_token_stream()?;
                result.push(ts);
            }
            Block::KVIEW(_) => {
                context.invalidate_static_context();
                let arguments = self
                    .content()
                    .parse::<TokenStream>()
                    .map_err(|err| error::CompileError::from_lex(self, err))?;
                let call = syn::parse2::<syn::ExprCall>(quote! { view #arguments })
                    .map_err(|err| error::CompileError::from_codegen(self, &err.to_string()))?;
                let mut arguments = call.args.iter();
                let name = arguments
                    .next()
                    .ok_or_else(|| error::CompileError::from_codegen(self, "Expected view name"))?;
                let child_context = arguments.next().ok_or_else(|| {
                    error::CompileError::from_codegen(self, "Expected view context")
                })?;
                let kind = arguments
                    .next()
                    .map_or_else(|| quote! { Self::kind() }, |kind| quote! { #kind });
                let view_root_mod_name = format_ident!("{}", context.options().mod_name());
                result.push(quote! {
                    {
                        #[allow(unused_imports)]
                        use sbolt::types::Context as _;
                        // Keep temporary arguments alive while evaluating them in source order.
                        let (output, _) = match (#name, (#child_context).as_mut_context(), #kind) {
                            (name, child_context, kind) => {
                                crate::#view_root_mod_name::render(name, Some(kind), child_context)
                            }
                        }?;
                        writer.write(&output);
                    }
                });
            }
            Block::KSECTION(_, _) => {
                context.invalidate_static_context();
                let ts = self.to_section_token_stream(context)?;
                result.push(ts);
            }
            Block::KUSE(_) => {
                context.invalidate_static_context();
                let use_ts = self.to_use_token_stream()?;
                result.push(use_ts);
            }
            Block::KLAYOUT(_) | Block::KMODEL(_) => {
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
        context: &mut CodeGenContext<'_>,
    ) -> result::Result<TokenStream> {
        if !matches!(self, Block::KROOT(_)) {
            return Err(error::CompileError::from_codegen(
                &self,
                "Wrong method call: couldn't generate code",
            ));
        }

        let root_span = self.span();
        let mut model_blocks = root_span
            .blocks()
            .iter()
            .filter(|block| matches!(block, Block::KMODEL(_)));
        let model_block = model_blocks.next();
        if model_blocks.next().is_some() {
            return Err(error::CompileError::from_codegen(
                self,
                "Multiple model directives found",
            ));
        }
        let model = model_block
            .map(|block| {
                syn::parse_str::<ModelDeclaration>(block.content().trim()).map_err(|err| {
                    error::CompileError::from_codegen(
                        block,
                        &format!("Invalid model declaration: {err}"),
                    )
                })
            })
            .transpose()?;
        let has_layout = root_span
            .blocks()
            .iter()
            .any(|b| matches!(b, Block::KLAYOUT(_)));
        if has_layout {
            context.invalidate_static_context();
        }
        let ts = self.to_token_stream(Some(self), context)?;
        let capacity_hint = context.capacity_hint();
        let get_model = match model {
            Some(declaration) => {
                let model_type = declaration.model_type;
                let model_key = declaration.key;
                quote! {
                    #[allow(unused_variables)]
                    let model = context.get_data::<::std::sync::Arc<#model_type>>(#model_key)
                        .cloned()
                        .ok_or_else(|| sbolt::types::error::RuntimeError::InvalidInput(
                            format!("Missing or incorrectly typed model at key '{}' for {}", #model_key, Self::name()),
                            format!("Store Arc<Model> under context key '{}'", #model_key),
                        ))?;
                }
            }
            None => quote! {},
        };
        let finish = match has_layout {
            true => {
                let view_root_mod_name = format_ident!("{}", context.options().mod_name());
                quote! {
                        match Self::layout() {
                            Some(layout) => {
                                for key in sbolt::types::resolve_layout_to_view_keys(&layout, &Self::name()) {
                                    if let Some(creator) = crate::#view_root_mod_name::resolve_view_creator(&key, Self::kind()) {
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
            }
            false => quote! { Ok(writer.into_string()) },
        };
        Ok(quote! {
            fn static_capacity_hint() -> usize {
                #capacity_hint
            }

            fn render(&self, context: &mut impl sbolt::types::Context) -> sbolt::types::result::RenderResult<String> {
                #get_model
                #[allow(unused_mut)]
                let mut writer = self.create_writer(None, Some(Self::static_capacity_hint().max(context.capacity_hint())));
                #(#ts)*
                #finish
            }
        })
    }
}
