use crate::codegen::parser::tokenizer::TokenStream;
use crate::{
    codegen::parser::{
        self,
        tokenizer::{self, Token},
    },
    types::{error, result},
};
use winnow::stream::Stream as _;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum Kind<'a> {
    // TODO:
    // USE: @use
    // Layout: @layout "layout_name"
    // Section: @section name{}
    // TODO: RenderSection: @render("name", is_required): this could be inline function call, render is defined in view.
    CODE(&'a str),
    INLINEDCODE(&'a str),
    CONTENT(&'a str),
    INLINEDCONTENT(&'a str),
    // intermediate kind.
    UNKNOWN(&'a str),
}

impl<'a> Default for Kind<'a> {
    fn default() -> Self {
        Self::UNKNOWN("")
    }
}

#[derive(Debug)]
pub(crate) struct Block<'a> {
    // block like use, section could have name.
    pub(crate) name: Option<String>,
    pub(crate) span: parser::Span<Kind<'a>>,
    pub(crate) blocks: Vec<Block<'a>>,
}

impl<'a> Default for Block<'a> {
    fn default() -> Self {
        Self {
            name: None,
            span: parser::Span::<Kind<'a>>::default(),
            blocks: vec![],
        }
    }
}

impl<'a> Block<'a> {
    #[cfg(test)]
    pub(crate) fn content(&self) -> &'a str {
        match &self.span.kind {
            Kind::CODE(content) => content,
            Kind::INLINEDCODE(content) => content,
            Kind::CONTENT(content) => content,
            Kind::INLINEDCONTENT(content) => content,
            Kind::UNKNOWN(content) => content,
        }
    }

    pub(crate) fn with_name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    pub(crate) fn with_span(&mut self, span: parser::Span<Kind<'a>>) -> &mut Self {
        self.span = span;
        self
    }

    pub(crate) fn push_block(&mut self, block: Block<'a>) -> &mut Self {
        // update container kind with first block.
        if matches!(self.span.kind, Kind::UNKNOWN(_)) {
            match &block.span.kind() {
                Kind::CODE(_) => {
                    self.span.kind = Kind::CODE("");
                }
                Kind::CONTENT(_) => {
                    self.span.kind = Kind::CONTENT("");
                }
                _ => { /* no-op */ }
            }
            self.span.start = block.span.start;
            self.span.end = block.span.end;
        } else {
            self.span.end = block.span.end;
        }
        self.blocks.push(block);
        self
    }
}

impl<'a> Block<'a> {
    pub(crate) fn parse_block_within_kind(
        source: &'a str,
        open_kind: tokenizer::Kind,
        close_kind: tokenizer::Kind,
        token_stream: &mut TokenStream,
        is_content: bool,
        is_inlined: bool,
    ) -> result::Result<Block<'a>> {
        // Assume the current token is the opening delimiter (either '(' or '{')
        _ = token_stream.next_token().ok_or_else(|| {
            let previous_token = token_stream.previous_tokens().last().copied();
            error::Error::from_parser(previous_token, "Expected opening delimiter").into()
        })?;

        let mut depth = 1;
        let mut result = Block::default();
        let start = token_stream.peek_token();
        let mut next_start = token_stream.peek_token();
        let mut end_token: Option<&Token> = None;
        while let Some(token) = token_stream.next_token() {
            match token.kind() {
                k if k == open_kind => {
                    depth += 1;
                }
                k if k == close_kind => {
                    depth -= 1;
                    if depth == 0 {
                        end_token = Some(token);
                        break;
                    }
                }
                tokenizer::Kind::AT => {
                    if is_inlined {
                        return Err(error::Error::from_parser(
                            Some(*token),
                            "Inlined block is not allowed to use '@' token",
                        ));
                    }

                    if is_content {
                        // transfer from content to code.
                        // 1. consume the tokens before this one as content block and switch to code block.
                        let content_block =
                            Self::create_block(source, &next_start, &Some(token), true, false)?;
                        result.push_block(content_block);

                        // 2. transfer to code.
                        let code_block = Self::parse_code(source, token, token_stream)?;
                        result.push_block(code_block);
                    } else {
                        // transfer from code to content.
                        // 1. consume the tokens before this one as code block and switch to content block.
                        let code_block =
                            Self::create_block(source, &next_start, &Some(token), false, false)?;
                        result.push_block(code_block);

                        // 2. transfer to content.
                        let content_block =
                            Self::parse_content(source, token, token_stream, false)?;
                        result.push_block(content_block);
                    }
                    next_start = token_stream.peek_token();
                }
                _ => {}
            }
        }

        if depth != 0 {
            return Err(error::Error::from_parser(
                Some(*start.unwrap()),
                "Unbalanced delimiters in block",
            ));
        }

        match next_start {
            Some(token) => {
                let block =
                    Self::create_block(source, &Some(token), &end_token, is_content, is_inlined)?;
                if start == next_start {
                    return Ok(block);
                } else {
                    result.push_block(block);
                }
            }
            None => { /* no-op */ }
        }

        Ok(result)
    }

    pub(crate) fn create_block(
        source: &'a str,
        start_token: &Option<&Token>,
        end_token: &Option<&Token>,
        is_content: bool,
        is_inlined: bool,
    ) -> result::Result<Block<'a>> {
        if start_token.is_none() {
            // not possible here.
            return Err(error::Error::from_parser(
                None,
                "Missing start or end token",
            ));
        }

        let start_token = start_token.unwrap();
        let start = start_token.range().start;
        let end = match end_token {
            Some(token) => token.range().start,
            None => source.len(),
        };

        if end <= start {
            return Err(error::Error::from_parser(
                end_token.cloned(),
                "Invalid token range",
            ));
        }

        let mut block = Block::default();
        match is_content {
            true => {
                if is_inlined {
                    block.with_span(parser::Span {
                        kind: Kind::INLINEDCONTENT(&source[start..end]),
                        start: start,
                        end: end,
                    })
                } else {
                    block.with_span(parser::Span {
                        kind: Kind::CONTENT(&source[start..end]),
                        start: start,
                        end: end,
                    })
                }
            }
            false => {
                if is_inlined {
                    block.with_span(parser::Span {
                        kind: Kind::INLINEDCODE(&source[start..end]),
                        start: start,
                        end: end,
                    })
                } else {
                    block.with_span(parser::Span {
                        kind: Kind::CODE(&source[start..end]),
                        start: start,
                        end: end,
                    })
                }
            }
        };
        Ok(block)
    }
}
