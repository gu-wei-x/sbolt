use crate::types::KWriter;
use crate::types::Writer;

impl Writer for KWriter {
    fn write(&mut self, content: &str) {
        match self {
            KWriter::KHtml(writer) => writer.write(content),
            KWriter::KJson(existing_content) => existing_content.write(content),
            KWriter::KText(existing_content) => existing_content.write(content),
        }
    }

    fn into_string(self) -> String {
        match self {
            KWriter::KHtml(writer) => writer.into_string(),
            KWriter::KJson(content) => content,
            KWriter::KText(content) => content,
        }
    }
}
