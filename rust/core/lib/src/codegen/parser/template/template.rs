use crate::codegen::parser::template::Fragment;
use crate::codegen::parser::tokenizer::{self, Tokenizer};
use winnow::stream::{Stream as _, TokenSlice};

pub(crate) struct Template<'a> {
    pub(crate) fragments: Vec<Fragment<'a>>,
    pub(crate) namespace: Option<String>,
}

impl<'a> Template<'a> {
    fn new(namespace: Option<String>) -> Self {
        Template {
            fragments: Vec::new(),
            namespace,
        }
    }
}

impl<'a> Template<'a> {
    pub(crate) fn from(
        source: &'a str,
        namespace: Option<String>,
    ) -> crate::types::result::Result<Self> {
        let tokenizer = Tokenizer::new(source);
        let tokens = tokenizer.into_vec();
        let mut token_stream = TokenSlice::new(&tokens);

        tokenizer::skip_whitespace_and_newline(&mut token_stream);
        let mut template = Template::new(namespace);
        while let Some(next_token) = token_stream.peek_token() {
            match next_token.kind() {
                tokenizer::Kind::EOF => break,
                tokenizer::Kind::NEWLINE => {
                    // consume newline.
                    token_stream.next_token();
                }
                tokenizer::Kind::AT => {
                    // consume @
                    let code_fragment = Self::code_from(source, &mut token_stream, next_token)?;
                    template.fragments.push(code_fragment);
                }
                _ => {
                    // consume content.
                    let content_fragment =
                        Self::content_from(source, &mut token_stream, next_token)?;
                    template.fragments.push(content_fragment);
                }
            }
        }

        Ok(template)
    }
}
