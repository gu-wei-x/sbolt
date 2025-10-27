pub(in crate::codegen) mod html;
pub(in crate::codegen) mod json;
mod span;
mod tokenizer;
mod types;

pub(in crate::codegen) use span::Span;
pub(in crate::codegen) use tokenizer::Token;
