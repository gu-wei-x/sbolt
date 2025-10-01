use crate::codegen::parser::template::ParseContext;
use crate::codegen::parser::tokenizer::Kind;
use crate::codegen::parser::tokenizer::Token;
use crate::codegen::parser::tokenizer::get_nth_token;
use winnow::stream::Stream as _;
use winnow::stream::TokenSlice;

pub(crate) fn _get_token_before_transfer<'a, F: Fn(Kind) -> bool>(
    source: &'a str,
    stream: &'a mut TokenSlice<Token>,
    parser_context: &ParseContext,
    skip_pred: F,
) -> Option<&'a Token> {
    while let Some(token) = stream.peek_token() {
        match token.kind() {
            Kind::EOF => return None,
            Kind::AT => {
                match parser_context.switch_if_possible(source, stream) {
                    Ok((false, _)) => {
                        // consume first @.
                        stream.next_token();
                        match stream.peek_token() {
                            Some(next_token) if next_token.kind() == Kind::AT => {
                                // @@, consume the second @.
                                stream.next_token();
                            }
                            _ => { /*no-ops */ }
                        }
                    }
                    Ok((true, _)) => break,
                    Err(_) => {
                        /* delay the error handling to the caller */
                        break;
                    }
                }
            }
            kind if skip_pred(kind) => {
                stream.next_token();
            }
            _ => break,
        }
    }
    stream.peek_token()
}

/// check if the token is @@escaped in the source string
pub(crate) fn is_token_escaped<'a>(stream: &'a TokenSlice<Token>) -> bool {
    let first = get_nth_token(stream, 0);
    let send = get_nth_token(stream, 1);
    match (first, send) {
        (Some(first), Some(send)) if first.kind() == Kind::AT && send.kind() == Kind::AT => true,
        _ => false,
    }
}
