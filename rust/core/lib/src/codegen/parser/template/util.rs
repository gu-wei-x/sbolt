use crate::codegen::parser::template::ParseContext;
use crate::codegen::parser::tokenizer::Kind;
use crate::codegen::parser::tokenizer::Token;
use winnow::stream::Stream as _;
use winnow::stream::TokenSlice;

pub(crate) fn get_token_before_transfer<'a, F: Fn(Kind) -> bool>(
    source: &'a str,
    stream: &'a mut TokenSlice<Token>,
    parser_context: &ParseContext,
    skip_pred: F,
) -> Option<&'a Token> {
    while let Some(token) = stream.peek_token() {
        let kind = token.kind();
        if kind == Kind::EOF {
            break;
        } else if kind == Kind::AT {
            match parser_context.should_switch(source, token, stream) {
                Ok(true) => break,
                _ => {
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
            }
        } else if skip_pred(kind) {
            stream.next_token();
        } else {
            break;
        }
    }

    stream.peek_token()
}
