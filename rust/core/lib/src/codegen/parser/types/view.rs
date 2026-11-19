use crate::codegen::parser::tokenizer::{self, TokenStream};
use crate::codegen::parser::types::context::ParseContext;
use crate::codegen::types::{Block, Span};
use crate::types::{error, result};
use proc_macro2::TokenStream as RustTokens;
use quote::quote;
use unicode_ident::{is_xid_continue, is_xid_start};
use winnow::stream::Stream as _;

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::types) fn parse_view<'s>(
        token_stream: &mut TokenStream<'_>,
        context: &ParseContext<'_, 's>,
    ) -> result::Result<Block<'s>> {
        let source = context.source();
        let directive = *token_stream.next_token().ok_or_else(|| {
            error::CompileError::from_parser(source, None, "Expected 'view' after '@'")
        })?;
        tokenizer::skip_whitespace_and_newline(token_stream);
        let opening = token_stream
            .peek_token()
            .filter(|token| token.kind() == tokenizer::Kind::OPARENTHESIS)
            .ok_or_else(|| {
                error::CompileError::from_parser(
                    source,
                    Some(directive),
                    "Expected '(' after 'view'",
                )
            })?;
        let start = opening.range().start;
        let end = argument_list_end(source, start).ok_or_else(|| {
            error::CompileError::from_parser(
                source,
                Some(directive),
                "Unclosed or invalid argument list in @view(name, context[, kind])",
            )
        })?;
        let arguments = source[start..end].parse::<RustTokens>().map_err(|err| {
            error::CompileError::from_parser(
                source,
                Some(directive),
                &format!("Invalid @view arguments: {err}"),
            )
        })?;
        let call = syn::parse2::<syn::ExprCall>(quote! { view #arguments }).map_err(|err| {
            error::CompileError::from_parser(
                source,
                Some(directive),
                &format!("Invalid @view arguments: {err}"),
            )
        })?;
        if !matches!(call.args.len(), 2 | 3) {
            return Err(error::CompileError::from_parser(
                source,
                Some(directive),
                "Expected two or three arguments in @view(name, context[, kind])",
            ));
        }
        let mut span = Span::new(source);
        while let Some(token) = token_stream.peek_token() {
            if token.range().start >= end {
                break;
            }
            span.push_token(*token);
            token_stream.next_token();
        }
        Ok(Block::new_view(span))
    }
}

fn argument_list_end(source: &str, start: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut index = start;
    let mut depth = 0;
    while let Some(&byte) = bytes.get(index) {
        match byte {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            b'"' => {
                index = quoted_end(source, index, b'"')?;
                continue;
            }
            b'\'' => {
                let next = source[index + 1..].chars().next()?;
                // A lifetime has no closing quote after its first character.
                if next == '\\' || bytes.get(index + 1 + next.len_utf8()) == Some(&b'\'') {
                    index = quoted_end(source, index, b'\'')?;
                    continue;
                }
            }
            b'/' if bytes.get(index + 1) == Some(&b'/') => {
                index += 2;
                while bytes.get(index).is_some_and(|&b| b != b'\n') {
                    index += 1;
                }
                continue;
            }
            b'/' if bytes.get(index + 1) == Some(&b'*') => {
                index += 2;
                let mut comments = 1;
                while comments > 0 {
                    match bytes.get(index..index + 2)? {
                        b"/*" => {
                            comments += 1;
                            index += 2;
                        }
                        b"*/" => {
                            comments -= 1;
                            index += 2;
                        }
                        _ => index += 1,
                    }
                }
                continue;
            }
            _ => {
                let ch = source[index..].chars().next()?;
                if ch != '_' && !is_xid_start(ch) && !ch.is_ascii_digit() {
                    index += ch.len_utf8();
                    continue;
                }
                let end = word_end(source, index);
                let word = &source[index..end];
                if matches!(word, "r" | "br" | "cr") {
                    let hashes = bytes[end..].iter().take_while(|&&b| b == b'#').count();
                    let quote = end + hashes;
                    if bytes.get(quote) == Some(&b'"') {
                        index = quote + 1;
                        loop {
                            let byte = *bytes.get(index)?;
                            index += 1;
                            if byte == b'"'
                                && bytes
                                    .get(index..index + hashes)
                                    .is_some_and(|suffix| suffix.iter().all(|&b| b == b'#'))
                            {
                                index = literal_suffix_end(source, index + hashes);
                                break;
                            }
                        }
                        continue;
                    }
                }
                if word == "r"
                    && bytes.get(end) == Some(&b'#')
                    && source[end + 1..]
                        .chars()
                        .next()
                        .is_some_and(|ch| ch == '_' || is_xid_start(ch))
                {
                    index = word_end(source, end + 1);
                } else {
                    index = end;
                }
                continue;
            }
        }
        index += 1;
    }
    None
}

fn word_end(source: &str, start: usize) -> usize {
    let tail = &source[start..];
    start
        + tail
            .char_indices()
            .find(|&(_, ch)| !is_xid_continue(ch))
            .map_or(tail.len(), |(offset, _)| offset)
}

fn literal_suffix_end(source: &str, start: usize) -> usize {
    if source[start..]
        .chars()
        .next()
        .is_some_and(|ch| ch == '_' || is_xid_start(ch))
    {
        word_end(source, start)
    } else {
        start
    }
}

fn quoted_end(source: &str, start: usize, quote: u8) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut index = start + 1;
    while let Some(&byte) = bytes.get(index) {
        index += 1;
        if byte == quote {
            return Some(literal_suffix_end(source, index));
        }
        if byte == b'\\' {
            index += 1;
        }
    }
    None
}
