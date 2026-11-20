use crate::codegen::consts;
use crate::codegen::parser::tokenizer::{self, TokenStream, get_nth_token};
use crate::codegen::types::Span;
use crate::codegen::types::{Block, ModelDeclaration};
use crate::types::{error, result};
use winnow::stream::Stream as _;

impl syn::parse::Parse for ModelDeclaration {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let key = if input.peek(syn::Ident)
            && input.peek2(syn::Token![:])
            && !input.peek2(syn::Token![::])
        {
            let key: syn::Ident = input.parse()?;
            input.parse::<syn::Token![:]>()?;
            key.to_string()
        } else {
            "model".to_string()
        };
        Ok(Self {
            key,
            model_type: input.parse()?,
        })
    }
}

impl<'a> Block<'a> {
    pub(in crate::codegen::parser::types) fn parse_directive<'s>(
        source: &'s str,
        directive: &str,
        token_stream: &mut TokenStream<'_>,
    ) -> result::Result<Block<'s>> {
        // token_stream starts with the directive.
        let start_token = match token_stream.peek_token() {
            Some(token) => match token.kind() {
                tokenizer::Kind::EXPRESSION if directive == &source[token.range()] => token,
                _ => {
                    return Err(error::CompileError::from_parser(
                        source,
                        Some(*token),
                        &format!("Expected '{directive}' after '@'"),
                    ));
                }
            },
            None => {
                return Err(error::CompileError::from_parser(
                    source,
                    None,
                    &format!("Expected '{directive}' after '@'"),
                ));
            }
        };

        // consume the directive token
        token_stream.next_token();

        // whitespace after directive token
        if !tokenizer::skip_whitespace(token_stream) {
            return Err(error::CompileError::from_parser(
                source,
                Some(*start_token),
                &format!("Expected whitespace name after '@{directive}'"),
            ));
        }

        // validate directive content.
        let next_token = get_nth_token(token_stream, 0);
        if None == next_token {
            return Err(error::CompileError::from_parser(
                source,
                Some(*start_token),
                &format!("Expected {directive} content after '@{directive}'"),
            ));
        }

        let mut span = Span::new(source);
        while let Some(token) = token_stream.peek_token() {
            match token.kind() {
                tokenizer::Kind::NEWLINE | tokenizer::Kind::SEMICOLON => {
                    if directive == consts::DIRECTIVE_KEYWORD_MODEL
                        && !span.content().trim().is_empty()
                        && syn::parse_str::<ModelDeclaration>(&span.content()).is_err()
                    {
                        // A Rust type can contain semicolons, e.g. [u8; 32].
                        span.push_token(*token);
                        token_stream.next_token();
                        continue;
                    }
                    // consume the end token.
                    token_stream.next_token();
                    break;
                }
                _ => {
                    span.push_token(*token);
                    token_stream.next_token();
                }
            }
        }

        let content = span.content();
        if content.trim().is_empty() {
            return Err(error::CompileError::from_parser(
                source,
                Some(*start_token),
                &format!("Expected {directive} content after '@{directive}'"),
            ));
        }

        match directive {
            consts::DIRECTIVE_KEYWORD_LAYOUT => Ok(Block::new_layout(span)),
            consts::DIRECTIVE_KEYWORD_MODEL => Ok(Block::new_model(span)),
            consts::DIRECTIVE_KEYWORD_USE => Ok(Block::new_use(span)),
            _ => Err(error::CompileError::from_parser(
                source,
                Some(*start_token),
                &format!("Unknown {directive} after '@{directive}'"),
            )),
        }
    }
}
