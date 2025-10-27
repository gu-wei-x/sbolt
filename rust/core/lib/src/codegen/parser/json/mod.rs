#![allow(dead_code)]

use crate::types::result;
mod object;
mod parser;

#[cfg(test)]
mod tests;

pub(in crate::codegen) fn parse_json<'s>(raw_content: &'s str) -> result::Result<object::JObject> {
    let mut state_machine = parser::StateMachine::new(raw_content);
    state_machine.process()
}
