use crate::{
    codegen::{
        consts,
        parser::{
            Token,
            template::ParseContext,
            tokenizer::{self, TokenStream},
        },
    },
    types::{error, result},
};
use winnow::stream::Stream as _;

pub(crate) fn get_context_at(
    source: &str,
    start_token: &Token,
    token_stream: &mut TokenStream,
    from_context: ParseContext,
) -> result::Result<ParseContext> {
    if start_token.kind() != tokenizer::Kind::AT {
        return Err(error::Error::from_parser(
            Some(*start_token),
            "Expected '@' token to start context extraction.",
        ));
    }

    let offset = match token_stream.peek_token() {
        Some(token) => {
            if token.range() == start_token.range() {
                1
            } else {
                // already consumed '@' token
                0
            }
        }
        _ => {
            return Ok(from_context);
        }
    };

    // check token after @, only swith context if legal,
    // else keeps current context unchanged to fail at compilation stage.
    match token_stream.offset_at(offset) {
        Ok(offset) => {
            if let Some(next_next_token) = token_stream.iter_offsets().nth(offset) {
                match next_next_token.1.kind() {
                    tokenizer::Kind::EXPRESSION => {
                        let exp = &source[next_next_token.1.range()];
                        match exp {
                            consts::DIRECTIVE_KEYWORD_USE => {
                                if from_context == ParseContext::Content {
                                    return Ok(ParseContext::Code);
                                }
                            }
                            consts::DIRECTIVE_KEYWORD_LAYOUT => {
                                // layout can only be used in content context.
                                if from_context == ParseContext::Content {
                                    return Ok(ParseContext::Code);
                                }
                            }
                            consts::KEYWORD_SECTION => {
                                // don't switch context
                                return Ok(from_context);
                            }
                            _ => {
                                // inlined
                                if from_context == ParseContext::Content {
                                    return Ok(ParseContext::Code);
                                } else {
                                    return Ok(ParseContext::Content);
                                }
                            }
                        }
                    }
                    tokenizer::Kind::OPARENTHESIS => {
                        // @(), inlined.
                        if from_context == ParseContext::Content {
                            return Ok(ParseContext::Code);
                        } else {
                            return Ok(ParseContext::Content);
                        }
                    }
                    tokenizer::Kind::OCURLYBRACKET => {
                        // @{}, block.
                        if from_context == ParseContext::Content {
                            return Ok(ParseContext::Code);
                        } else {
                            return Ok(ParseContext::Content);
                        }
                    }
                    _ => {
                        // don't switch context
                        return Ok(from_context);
                    }
                }
            }
        }
        _ => {
            // don't switch context
            return Ok(from_context);
        }
    }

    // don't switch context
    Ok(from_context)
}
