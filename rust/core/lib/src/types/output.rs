use super::{WriteTo, Writer};
use std::fmt::Display;

#[doc(hidden)]
pub struct HtmlEscapingWriter<W>(pub W);

impl<W> HtmlEscapingWriter<W> {
    pub fn into_inner(self) -> W {
        self.0
    }
}

impl<W: Writer> Writer for HtmlEscapingWriter<W> {
    fn write(&mut self, content: &str) {
        let mut start = 0;
        for (index, ch) in content.char_indices() {
            let escaped = match ch {
                '&' => "&amp;",
                '<' => "&lt;",
                '>' => "&gt;",
                '"' => "&quot;",
                '\'' => "&#39;",
                _ => continue,
            };
            self.0.write(&content[start..index]);
            self.0.write(escaped);
            start = index + ch.len_utf8();
        }
        self.0.write(&content[start..]);
    }

    fn write_raw_formatted(&mut self, args: std::fmt::Arguments<'_>) {
        self.0.write_raw_formatted(args);
    }

    fn into_string(self) -> String {
        self.0.into_string()
    }
}

/// Escape HTML text and quoted attribute values, not JavaScript, CSS, or URLs.
pub fn escape_html(value: impl Display) -> String {
    let mut writer = HtmlEscapingWriter(String::new());
    writer.write_formatted(format_args!("{value}"));
    writer.into_string()
}

/// Explicitly trusted HTML for an HTML-autoescaped expression.
///
/// This wrapper does not implement `Display`; use it only in HTML templates.
pub struct Raw<T>(T);

/// Bypass HTML escaping for trusted markup. This does not sanitize the value.
pub fn raw<T: Display>(value: T) -> Raw<T> {
    Raw(value)
}

impl<T: Display> WriteTo for Raw<T> {
    fn write_to(&self, writer: &mut impl Writer) {
        writer.write_raw_formatted(format_args!("{}", self.0));
    }
}

/// Serialize JSON with escapes suitable for a script value or JSON script element.
///
/// For a quoted HTML attribute, HTML-escape the result as well. This is not
/// suitable for event handlers or interpolation inside JavaScript strings.
pub fn json<T: serde::Serialize + ?Sized>(value: &T) -> super::result::RenderResult<String> {
    let mut output = String::new();
    write_json(value, &mut output)?;
    Ok(output)
}

/// Validate pre-serialized JSON for insertion as a JSON value, without quoting it.
pub fn raw_json(value: &str) -> super::result::RenderResult<&serde_json::value::RawValue> {
    serde_json::from_str(value).map_err(|error| {
        super::error::RuntimeError::InvalidInput("Invalid raw JSON".to_string(), error.to_string())
    })
}

#[doc(hidden)]
pub fn write_json<T: serde::Serialize + ?Sized>(
    value: &T,
    writer: &mut impl Writer,
) -> super::result::RenderResult<()> {
    struct Adapter<'a, W>(&'a mut W);

    impl<W: Writer> std::io::Write for Adapter<'_, W> {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            let content = std::str::from_utf8(bytes)
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
            let mut start = 0;
            for (index, ch) in content.char_indices() {
                let escaped = match ch {
                    '<' => "\\u003c",
                    '>' => "\\u003e",
                    '&' => "\\u0026",
                    '\'' => "\\u0027",
                    '\u{2028}' => "\\u2028",
                    '\u{2029}' => "\\u2029",
                    _ => continue,
                };
                self.0.write(&content[start..index]);
                self.0.write(escaped);
                start = index + ch.len_utf8();
            }
            self.0.write(&content[start..]);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    serde_json::to_writer(Adapter(writer), value).map_err(|error| {
        super::error::RuntimeError::InvalidInput(
            "JSON serialization failed".to_string(),
            error.to_string(),
        )
    })
}
