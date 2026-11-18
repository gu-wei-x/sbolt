mod whitespace;

#[cfg(test)]
mod doc;
#[cfg(test)]
mod node;
#[cfg(test)]
mod parser;
#[cfg(test)]
mod tests;

pub(in crate::codegen) use whitespace::compact_html_whitespace;

#[cfg(test)]
use crate::types::result;
// Keep the legacy DOM parser test-only; code generation preserves source spelling.
#[cfg(test)]
pub(in crate::codegen) fn parse_html<'s>(raw_html: &'s str) -> result::Result<doc::HtmlDocument> {
    doc::HtmlDocument::parse(raw_html)
}
