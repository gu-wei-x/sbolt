use crate::codegen::parser::template::Template;
use crate::codegen::parser::template::{self, Fragment};
use crate::codegen::parser::tokenizer::{self, Token, TokenStream};
use crate::types::error; // Add this import if your error type is defined here
use crate::types::result;
use winnow::stream::Stream as _;

impl<'a> Template<'a> {
    pub(crate) fn code_from(
        source: &'a str,
        token_stream: &mut TokenStream,
        start_token: &Token,
    ) -> result::Result<Fragment<'a>> {
        if start_token.kind() != tokenizer::Kind::AT {
            return error::Error::from_parser("Expected '@' token").into();
        }

        // consume @<expr:whitespace>
        token_stream.next_token();
        tokenizer::skip_whitespace(token_stream);
        match token_stream.peek_token() {
            Some(token) => match token.kind() {
                tokenizer::Kind::OPARENTHESIS => Self::create_fragment_with_kind(
                    source,
                    tokenizer::Kind::OPARENTHESIS,
                    tokenizer::Kind::CPARENTHESIS,
                    token_stream,
                ),
                tokenizer::Kind::OCURLYBRACKET => Self::create_fragment_with_kind(
                    source,
                    tokenizer::Kind::OCURLYBRACKET,
                    tokenizer::Kind::CCURLYBRACKET,
                    token_stream,
                ),
                _ => Self::create_inlined_code_fragment(source, token, token_stream),
            },
            _ => {
                println!("*****************{:?}", token_stream.peek_token());
                // not code block but a single @.
                Ok(Fragment {
                    kind: template::Kind::CONTENT(&source[start_token.range()]),
                    start: start_token.start,
                    end: start_token.end,
                })
            }
        }
    }
}

impl<'a> Template<'a> {
    fn create_fragment_with_kind(
        source: &'a str,
        open_kind: tokenizer::Kind,
        close_kind: tokenizer::Kind,
        token_stream: &mut TokenStream,
    ) -> result::Result<Fragment<'a>> {
        // Assume the current token is the opening delimiter (either '(' or '{')
        let start_token = token_stream
            .next_token()
            .ok_or_else(|| error::Error::from_parser("Expected opening delimiter").into())?;

        let mut depth = 1;
        let start = start_token.range().start;
        let mut end = start_token.range().end;
        while let Some(token) = token_stream.next_token() {
            match token.kind() {
                k if k == open_kind => {
                    depth += 1;
                }
                k if k == close_kind => {
                    depth -= 1;
                    if depth == 0 {
                        end = token.range().end;
                        break;
                    }
                }
                _ => {}
            }
            end = token.range().end;
        }

        if depth != 0 {
            return error::Error::from_parser("Unbalanced delimiters in code block").into();
        }

        Ok(Fragment {
            kind: template::Kind::CODE(&source[start + 1..end - 1]),
            start: start + 1,
            end: end - 1,
        })
    }

    fn create_inlined_code_fragment(
        source: &'a str,
        token: &Token,
        token_stream: &mut TokenStream,
    ) -> result::Result<Fragment<'a>> {
        if let Some(end_token) = tokenizer::get_next_token_if(token_stream, |k| {
            !vec![
                tokenizer::Kind::WHITESPACE,
                tokenizer::Kind::NEWLINE,
                tokenizer::Kind::EOF,
                tokenizer::Kind::EXPRESSION,
            ]
            .contains(&k)
        }) {
            token_stream.next_token(); // consume end_token
            Ok(Fragment {
                kind: template::Kind::CODE(&source[token.range().start..end_token.range().end]),
                start: token.range().start,
                end: end_token.range().end,
            })
        } else {
            // TODO: consume all left.
            token_stream.next_token(); // consume end_token
            Ok(Fragment {
                kind: template::Kind::CONTENT(&source[token.range().start..source.len()]),
                start: token.range().start,
                end: source.len(),
            })
        }
    }
}
