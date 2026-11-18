use crate::codegen::compiler::context::CodeGenContext;
use crate::codegen::parser::Token;
use crate::codegen::types::Block;
use crate::types::{Location, error, result};
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    pub(in crate::codegen::compiler::types) fn is_static_output(&self) -> bool {
        match self {
            Block::KCOMMENT(_) => true,
            Block::KCONTENT(span) | Block::KINLINEDCONTENT(span) if span.is_simple() => true,
            Block::KCONTENT(span) => span.blocks().iter().all(Self::is_static_output),
            _ => false,
        }
    }

    fn collect_static_tokens(&self, tokens: &mut Vec<Token>) {
        match self {
            Block::KCONTENT(span) | Block::KINLINEDCONTENT(span) if span.is_simple() => {
                tokens.extend_from_slice(span.tokens());
            }
            Block::KROOT(span) | Block::KCONTENT(span) => {
                for block in span.blocks() {
                    block.collect_static_tokens(tokens);
                }
            }
            _ => {}
        }
    }

    pub(in crate::codegen::compiler::types) fn remap_static_error(
        &self,
        error: error::CompileError,
    ) -> error::CompileError {
        let error::CompileError::Parser(Some(location), message, _) = error else {
            return error;
        };
        let source = self.span().source();
        let mut tokens = Vec::new();
        self.collect_static_tokens(&mut tokens);
        let content: String = tokens.iter().map(|token| &source[token.range()]).collect();
        let mut offset = content
            .split_inclusive('\n')
            .take(location.line)
            .map(str::len)
            .sum::<usize>()
            + location.column;
        let source_offset = tokens
            .iter()
            .find_map(|token| {
                let range = token.range();
                if offset < range.len() {
                    Some(range.start + offset)
                } else {
                    offset -= range.len();
                    None
                }
            })
            // EOF belongs after any trailing template comments.
            .unwrap_or(source.len());
        let prefix = &source[..source_offset];
        let line = prefix
            .bytes()
            .enumerate()
            .filter(|(index, byte)| {
                *byte == b'\n'
                    || (*byte == b'\r' && prefix.as_bytes().get(index + 1) != Some(&b'\n'))
            })
            .count();
        let column = prefix.rsplit(['\r', '\n']).next().unwrap_or("").len();
        error::CompileError::Parser(
            Some(Location::new(line, column)),
            message,
            source[source_offset..].chars().take(10).collect(),
        )
    }

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
        context: &CodeGenContext<'_>,
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
                    result.push(Self::static_write(&pending, writes, context)?);
                    pending.clear();
                    writes = 0;
                }
            }
            result.extend(block.to_token_stream(from, context)?);
        }
        if writes > 0 {
            result.push(Self::static_write(&pending, writes, context)?);
        }
        Ok(result)
    }

    fn static_write(
        content: &str,
        writes: usize,
        context: &CodeGenContext<'_>,
    ) -> result::Result<TokenStream> {
        Ok(match context.static_content(content, writes)? {
            Some(content) => quote! { writer.write(#content); },
            None => TokenStream::new(),
        })
    }

    pub(in crate::codegen::compiler::types) fn to_content_token_stream(
        &self,
        context: &CodeGenContext<'_>,
    ) -> result::Result<TokenStream> {
        if !matches!(self, Block::KCONTENT(_)) {
            return Err(error::CompileError::from_codegen(
                &self,
                "Wrong method call: couldn't generate code",
            ));
        }

        let content_span = self.span();
        if content_span.is_simple() {
            Self::static_write(&content_span.content(), 1, context)
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
        context: &CodeGenContext<'_>,
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
            Self::static_write(&content_span.content(), 1, context)
        } else {
            Err(error::CompileError::from_codegen(
                &self,
                "Inlined content with nested blocks is not supported",
            ))
        }
    }
}
