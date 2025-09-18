use crate::codegen::parser::template::Template;
use crate::codegen::parser::template::{self, Fragment};
use crate::codegen::parser::tokenizer::{self, Token, TokenStream};
use crate::types::result;

impl<'a> Template<'a> {
    pub(crate) fn content_from(
        source: &'a str,
        token_stream: &mut TokenStream,
        start_token: &Token,
    ) -> result::Result<Fragment<'a>> {
        if let Some(end_token) =
            tokenizer::get_next_token_if(token_stream, |k| !vec![tokenizer::Kind::AT].contains(&k))
        {
            Ok(Fragment {
                kind: template::Kind::CONTENT(
                    &source[start_token.range().start..end_token.range().start],
                ),
                start: start_token.range().start,
                end: end_token.range().start,
            })
        } else {
            Ok(Fragment {
                kind: template::Kind::CONTENT(&source[start_token.range().start..source.len()]),
                start: start_token.range().start,
                end: source.len(),
            })
        }
    }
}
