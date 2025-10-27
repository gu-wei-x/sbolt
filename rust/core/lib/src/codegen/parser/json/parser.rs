use winnow::stream::{Stream as _, TokenSlice};

use crate::{
    codegen::parser::{
        json::object::{self, JObject},
        tokenizer::{self, TokenStream, Tokenizer},
    },
    types::error,
    types::result,
};

#[derive(Clone, Copy, Debug)]
pub(in crate::codegen::parser::json) enum State {
    INIT,
    MAPSTART,
    ARRAYSTART,
    ITEMNAME,
    ITEMVALUE,
    ITEM,
    DONE,
}

#[derive(Debug)]
pub(in crate::codegen::parser::json) struct StateMachine<'s> {
    // statck of nodes needs to be parsed.
    jobjs: Vec<JObject>,
    pending_name: String,
    source: &'s str,
    state: State,
}

impl<'s> StateMachine<'s> {
    pub(in crate::codegen::parser::json) fn new(source: &'s str) -> Self {
        Self {
            jobjs: vec![],
            pending_name: "".into(),
            source: source,
            state: State::INIT,
        }
    }

    pub(in crate::codegen::parser::json) fn process(&mut self) -> result::Result<object::JObject> {
        let tokenizer = Tokenizer::new(self.source);
        let tokens = tokenizer.into_vec();
        let mut token_stream = TokenSlice::new(&tokens);
        let mut object = object::JObject::default();
        while let Some(token) = token_stream.peek_token() {
            if token.kind() == tokenizer::Kind::EOF {
                self.transit_to(State::DONE, &mut object);
                break;
            }
            match self.state {
                State::INIT => self.process_with_init(&mut token_stream, &mut object)?,
                State::MAPSTART => todo!(),
                State::ARRAYSTART => todo!(),
                State::ITEMNAME => todo!(),
                State::ITEMVALUE => todo!(),
                State::ITEM => todo!(),
                State::DONE => {
                    self.transit_to(State::DONE, &mut object);
                    break;
                }
            }
        }

        // todo: unwrap the root object.
        Ok(object)
    }
}

impl<'s> StateMachine<'s> {
    fn transit_to(&mut self, _state: State, _object: &mut object::JObject) {}

    fn process_with_init(
        &mut self,
        token_stream: &mut TokenStream,
        object: &mut object::JObject,
    ) -> result::Result<()> {
        // we know there is valid token;
        let token = token_stream.peek_token().unwrap();
        match token.kind() {
            tokenizer::Kind::OCURLYBRACKET => {
                // consmue '{'
                token_stream.next_token();
                self.transit_to(State::MAPSTART, object);
            }
            tokenizer::Kind::DQMARK => {
                token_stream.next_token();
                self.transit_to(State::ITEMNAME, object);
            }
            tokenizer::Kind::WHITESPACE | tokenizer::Kind::NEWLINE => {
                token_stream.next_token();
            }
            _ => {
                token_stream.next_token();
                return Err(error::CompileError::from_parser(
                    self.source,
                    Some(*token),
                    "Ilegal char",
                ));
            }
        }

        Ok(())
    }
}
