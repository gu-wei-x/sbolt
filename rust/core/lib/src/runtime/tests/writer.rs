#![cfg(test)]
use crate::types::{DisplayWriteTo as _, OptionDisplayWriteTo as _, WriteToRef};
use crate::types::{HtmlWriter, KWriter, WriteTo, Writer, display};

#[test]
fn automatic_rendering_prefers_specialization_and_falls_back_to_display() {
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
    output = WriteToRef(&Specialized).write_to(output);
    output = WriteToRef(&RenderOnly).write_to(output);
    output = WriteToRef(&FragmentedDisplay).write_to(output);
    let dynamic: &dyn std::fmt::Display = &FragmentedDisplay;
    output = WriteToRef(dynamic).write_to(output);
    output = WriteToRef(&1.25).write_to(output);
    output = WriteToRef(&'&').write_to(output);
    output.write("|");
    output = WriteToRef(&Some(Specialized)).write_to(output);
    output = WriteToRef(&Some(RenderOnly)).write_to(output);
    output = WriteToRef(&None::<Specialized>).write_to(output);
    output = WriteToRef(&Some(Some(7))).write_to(output);
    assert_eq!(
        output,
        "specializedrender-only<em>42</em><em>42</em>1.25&|specializedrender-only7"
    );
}

#[test]
fn optional_display_values_support_borrowing_and_html_escaping() {
    let mut value = Some(FragmentedDisplay);
    let borrowed = &value;
    let nested = &borrowed;
    let mut output = String::new();
    output = WriteToRef(&value).write_to(output);
    output = WriteToRef(&nested).write_to(output);
    output = WriteToRef(&(&mut value)).write_to(output);
    output = WriteToRef(&None::<FragmentedDisplay>).write_to(output);
    assert_eq!(output, "<em>42</em>".repeat(3));

    let mut missing = None::<FragmentedDisplay>;
    output = WriteToRef(&(&mut missing)).write_to(output);
    assert_eq!(output, "<em>42</em>".repeat(3));

    let mut html = crate::types::HtmlEscapingWriter(String::new());
    html = WriteToRef(&value).write_to(html);
    html = WriteToRef(&Some(crate::utils::raw("<b>trusted</b>"))).write_to(html);
    assert_eq!(html.into_string(), "&lt;em&gt;42&lt;/em&gt;<b>trusted</b>");
}

#[test]
fn display_fallback_uses_writer_formatting() {
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
    writer = WriteToRef(&FragmentedDisplay).write_to(writer);
    assert!(writer.formatted);
    assert_eq!(writer.into_string(), "<em>42</em>");
}

#[test]
fn json_helpers_stream_unicode_and_preserve_validated_fragments()
-> crate::types::result::RenderResult<()> {
    use crate::utils::{json, raw_json};

    let value = "\"\\\n\r\t\0<>&'\u{2028}\u{2029}\u{e9}\u{1f642}";
    assert_eq!(
        json(&value)?,
        concat!(
            r#""\"\\\n\r\t\u0000\u003c\u003e\u0026\u0027\u2028\u2029"#,
            "\u{e9}\u{1f642}",
            "\""
        )
    );
    let fragment = raw_json(r#"{"text":"</script>"}"#)?;
    assert_eq!(json(fragment)?, r#"{"text":"\u003c/script\u003e"}"#);
    for invalid in ["{", "true false"] {
        assert!(matches!(
            raw_json(invalid),
            Err(crate::types::error::RuntimeError::InvalidInput(_, _))
        ));
    }
    Ok(())
}

#[test]
fn json_custom_serializers_preserve_structure_and_escape_keys_and_values()
-> crate::types::result::RenderResult<()> {
    struct CustomValue<'a> {
        message: &'a str,
        count: u8,
    }

    impl serde::Serialize for CustomValue<'_> {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use serde::ser::SerializeStruct as _;

            let mut state = serializer.serialize_struct("CustomValue", 2)?;
            state.serialize_field("<message>", self.message)?;
            state.serialize_field("count", &self.count)?;
            state.end()
        }
    }

    let value = CustomValue {
        message: "</script>&'\"\\\n\u{2028}\u{2029}",
        count: 42,
    };
    let expected =
        r#"{"\u003cmessage\u003e":"\u003c/script\u003e\u0026\u0027\"\\\n\u2028\u2029","count":42}"#;
    assert_eq!(crate::utils::json(&value)?, expected);
    let mut output = String::from("prefix|");
    crate::types::write_json(&value, &mut output)?;
    assert_eq!(output, format!("prefix|{expected}"));
    Ok(())
}

#[test]
fn json_custom_serializer_errors_propagate() {
    struct FailingValue;

    impl serde::Serialize for FailingValue {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use serde::ser::{Error as _, SerializeSeq as _};

            let mut sequence = serializer.serialize_seq(Some(2))?;
            sequence.serialize_element("written before failure")?;
            Err(S::Error::custom("intentional serialization failure"))
        }
    }

    assert!(matches!(
        crate::utils::json(&FailingValue),
        Err(crate::types::error::RuntimeError::InvalidInput(ref message, ref detail))
            if message == "JSON serialization failed"
                && detail.contains("intentional serialization failure")
    ));
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
    for mut writer in [
        KWriter::KHtml(HtmlWriter::new(None)),
        KWriter::KJson(String::new()),
        KWriter::KText(String::new()),
    ] {
        assert_formatted_output(&mut writer);
        writer.write("reused");
        assert_eq!(writer.into_string(), "reused");
    }
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
