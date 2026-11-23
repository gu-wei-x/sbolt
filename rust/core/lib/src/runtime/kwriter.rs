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

    fn write_formatted(&mut self, args: std::fmt::Arguments<'_>) {
        match self {
            KWriter::KHtml(writer) => writer.write_formatted(args),
            KWriter::KJson(content) | KWriter::KText(content) => {
                Writer::write_formatted(content, args)
            }
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

impl Writer for &mut KWriter {
    fn write(&mut self, content: &str) {
        (**self).write(content);
    }

    fn write_formatted(&mut self, args: std::fmt::Arguments<'_>) {
        (**self).write_formatted(args);
    }

    fn into_string(self) -> String {
        match self {
            KWriter::KHtml(writer) => std::mem::take(&mut writer.content),
            KWriter::KJson(content) | KWriter::KText(content) => std::mem::take(content),
        }
    }
}
