mod doc;
mod node;
mod parser;

#[cfg(test)]
mod tests;

pub(in crate::codegen) fn parse_html<'s>(raw_html: &'s str) -> doc::HtmlDocument {
    doc::HtmlDocument::parse(raw_html)
}
