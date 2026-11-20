#![cfg(test)]
use crate::types::{DisplayWriteTo as _, WriteToRef};
use crate::types::{HtmlWriter, KWriter, WriteTo, Writer, display};

#[test]
fn automatic_rendering_prefers_specialization_and_falls_back_to_display() {
    #[allow(unused_imports)]
    use crate::types::ToStringWriteTo as _;

    struct Specialized;
    impl WriteTo for Specialized {
        fn write_to(&self, writer: &mut impl Writer) {
            writer.write("specialized");
        }
    }
    impl std::fmt::Display for Specialized {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("display")
        }
    }

    struct RenderOnly;
    impl WriteTo for RenderOnly {
        fn write_to(&self, writer: &mut impl Writer) {
            writer.write("render-only");
        }
    }

    let mut output = String::new();
    WriteToRef(&Specialized).__sbolt_write_to(&mut output);
    WriteToRef(&RenderOnly).__sbolt_write_to(&mut output);
    WriteToRef(&FragmentedDisplay).__sbolt_write_to(&mut output);
    let dynamic: &dyn std::fmt::Display = &FragmentedDisplay;
    WriteToRef(dynamic).__sbolt_write_to(&mut output);
    WriteToRef(&1.25).__sbolt_write_to(&mut output);
    WriteToRef(&'&').__sbolt_write_to(&mut output);
    assert_eq!(output, "specializedrender-only<em>42</em><em>42</em>1.25&");
}

#[test]
fn legacy_to_string_values_support_borrowing_and_dereferencing() {
    use crate::types::ToStringWriteTo as _;

    struct LegacyValue;
    impl ToString for LegacyValue {
        fn to_string(&self) -> String {
            "<legacy>".to_owned()
        }
    }
    impl LegacyValue {
        fn write_to(&self, writer: &mut impl Writer) {
            writer.write("inherent");
        }
    }

    let value = LegacyValue;
    let borrowed = &value;
    let shared = std::sync::Arc::new(LegacyValue);
    let dynamic: &dyn ToString = &value;
    let mut output = String::new();
    WriteToRef(&value).__sbolt_write_to(&mut output);
    WriteToRef(&borrowed).__sbolt_write_to(&mut output);
    WriteToRef(&shared).__sbolt_write_to(&mut output);
    WriteToRef(dynamic).__sbolt_write_to(&mut output);
    assert_eq!(output, "<legacy>".repeat(4));
    output.clear();
    value.write_to(&mut output);
    assert_eq!(output, "inherent");
}

#[test]
fn display_streaming_precedes_the_legacy_to_string_fallback() {
    #[allow(unused_imports)]
    use crate::types::ToStringWriteTo as _;

    struct StreamingWriter {
        output: String,
        formatted: bool,
    }
    impl Writer for StreamingWriter {
        fn write(&mut self, content: &str) {
            self.output.push_str(content);
        }

        fn write_formatted(&mut self, args: std::fmt::Arguments<'_>) {
            self.formatted = true;
            self.output.write_formatted(args);
        }

        fn into_string(self) -> String {
            self.output
        }
    }

    let mut writer = StreamingWriter {
        output: String::new(),
        formatted: false,
    };
    WriteToRef(&FragmentedDisplay).__sbolt_write_to(&mut writer);
    assert!(writer.formatted);
    assert_eq!(writer.into_string(), "<em>42</em>");
}

#[test]
fn specialized_booleans_append_without_a_display_adapter() {
    let mut output = String::from("prefix:");
    output.write_value(&true);
    output.write("|");
    output.write_value(&false);
    assert_eq!(output, "prefix:true|false");
}

#[test]
fn specialized_values_preserve_integer_limits_and_raw_strings() {
    let mut output = String::new();
    macro_rules! check_integers {
        ($($integer:ty),* $(,)?) => {
            $(
                for value in [<$integer>::MIN, 0, <$integer>::MAX] {
                    output.clear();
                    value.write_to(&mut output);
                    assert_eq!(output, value.to_string(), stringify!($integer));
                }
            )*
        };
    }
    check_integers!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    );
    output.clear();
    let owned = String::from("<b>&\"raw\"</b>");
    owned.write_to(&mut output);
    owned.as_str().write_to(&mut output);
    display(FragmentedDisplay).write_to(&mut output);
    display(true).write_to(&mut output);
    display(1.25).write_to(&mut output);
    assert_eq!(output, "<b>&\"raw\"</b><b>&\"raw\"</b><em>42</em>true1.25");
}

struct FragmentedDisplay;

impl std::fmt::Display for FragmentedDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<em>")?;
        write!(f, "{}", 42)?;
        f.write_str("</em>")
    }
}

fn assert_formatted_output(mut writer: impl Writer) {
    let owned = String::from("team & \"name\"");
    let borrowed = owned.as_str();
    writer.write("prefix:");
    writer.write_formatted(format_args!(
        "{}|{}|{}|{}|{}",
        -123, borrowed, owned, FragmentedDisplay, ""
    ));
    writer.write_formatted(format_args!("|{}", owned));
    assert_eq!(
        writer.into_string(),
        "prefix:-123|team & \"name\"|team & \"name\"|<em>42</em>||team & \"name\""
    );
}

#[test]
fn formatted_output_preserves_values_for_all_builtin_writers() {
    for hint in [None, Some(256)] {
        assert_formatted_output(HtmlWriter::new(hint));
        assert_formatted_output(KWriter::KHtml(HtmlWriter::new(hint)));
    }
    assert_formatted_output(String::new());
    assert_formatted_output(KWriter::KJson(String::new()));
    assert_formatted_output(KWriter::KText(String::new()));
}

#[test]
fn custom_writer_supports_formatted_output_without_an_override() {
    struct CustomWriter(String);

    impl Writer for CustomWriter {
        fn write(&mut self, content: &str) {
            self.0.push_str(content);
        }

        fn into_string(self) -> String {
            self.0
        }
    }

    assert_formatted_output(CustomWriter(String::new()));
}

#[test]
#[should_panic(expected = "formatting template value failed")]
fn formatting_errors_are_not_silently_ignored() {
    struct InvalidDisplay;

    impl std::fmt::Display for InvalidDisplay {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Err(std::fmt::Error)
        }
    }

    HtmlWriter::new(None).write_formatted(format_args!("{}", InvalidDisplay));
}

#[test]
fn html_writer() {
    let content = "hell world!";
    for hint in [None, Some(0), Some(1), Some(64)] {
        let mut html_writer = HtmlWriter::new(hint);
        if let Some(capacity) = hint {
            assert!(html_writer.content.capacity() >= capacity);
        }
        html_writer.write(content);
        html_writer.writeln("");
        html_writer.writefn(|| "test".to_string());
        assert_eq!(
            html_writer.into_string(),
            format!("{}\n{}", content, "test")
        );
    }
}

#[test]
fn html_kwriter() {
    let content = "hell world!";
    let html_writer = HtmlWriter::new(None);
    let mut kwriter = KWriter::KHtml(html_writer);
    kwriter.write(content);
    assert_eq!(kwriter.into_string(), content)
}

#[test]
fn string_writer() {
    let content = "hell world!";
    let mut string_writer = String::new();
    string_writer.write(content);
    string_writer.writeln("");
    string_writer.writefn(|| "test".to_string());
    assert_eq!(
        string_writer.into_string(),
        format!("{}\n{}", content, "test")
    );
}

#[test]
fn string_kwriter() {
    let content = "hell world!";
    let string_writer = String::new();
    let mut kwriter = KWriter::KText(string_writer);
    kwriter.write(content);
    assert_eq!(kwriter.into_string(), content)
}

#[test]
fn json_kwriter() {
    let content = "hell world!";
    let string_writer = String::new();
    let mut kwriter = KWriter::KJson(string_writer);
    kwriter.write(content);
    assert_eq!(kwriter.into_string(), content)
}
