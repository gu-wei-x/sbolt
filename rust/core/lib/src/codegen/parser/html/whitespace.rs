pub(in crate::codegen) fn compact_html_whitespace(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut remaining = source;
    let mut previous_is_block = None;
    while let Some(start) = remaining.find('<') {
        let text = &remaining[..start];
        let tag_source = &remaining[start..];
        if tag_source
            .get(..15)
            .is_some_and(|s| s.eq_ignore_ascii_case("<!doctype html>"))
        {
            output.push_str(text);
            output.push_str(&tag_source[..15]);
            remaining = &tag_source[15..];
            previous_is_block = Some(true);
            continue;
        }
        let Some((length, tag, is_block)) = compact_tag(tag_source) else {
            // Unsupported syntax and raw-text containers end the known static region.
            output.push_str(remaining);
            return output;
        };
        match previous_is_block {
            Some(previous) if text.bytes().all(is_space) => {
                if !text.is_empty() && !previous && !is_block {
                    output.push(' ');
                }
            }
            _ => output.push_str(text),
        }
        output.push_str(&tag);
        remaining = &tag_source[length..];
        previous_is_block = Some(is_block);
    }
    output.push_str(remaining);
    output
}

fn is_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b'\x0c')
}

fn skip_space(bytes: &[u8], mut index: usize) -> usize {
    while bytes.get(index).is_some_and(|b| is_space(*b)) {
        index += 1;
    }
    index
}

fn is_block_tag(name: &str) -> bool {
    [
        "address",
        "article",
        "aside",
        "blockquote",
        "body",
        "caption",
        "center",
        "dd",
        "details",
        "dir",
        "div",
        "dl",
        "dt",
        "fieldset",
        "figcaption",
        "figure",
        "footer",
        "form",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "head",
        "header",
        "hgroup",
        "hr",
        "html",
        "li",
        "main",
        "menu",
        "nav",
        "ol",
        "p",
        "search",
        "section",
        "summary",
        "table",
        "tbody",
        "td",
        "tfoot",
        "th",
        "thead",
        "tr",
        "ul",
    ]
    .iter()
    .any(|block| name.eq_ignore_ascii_case(block))
}

fn compact_tag(source: &str) -> Option<(usize, String, bool)> {
    let bytes = source.as_bytes();
    let closing = bytes.get(1) == Some(&b'/');
    let name_start = if closing { 2 } else { 1 };
    if !bytes.get(name_start)?.is_ascii_alphabetic() {
        return None;
    }
    let mut index = name_start;
    while bytes
        .get(index)
        .is_some_and(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b':' | b'.'))
    {
        index += 1;
    }
    let name = &source[name_start..index];
    let is_block = is_block_tag(name);
    if [
        "pre",
        "listing",
        "dialog",
        "textarea",
        "script",
        "style",
        "title",
        "xmp",
        "iframe",
        "noembed",
        "noframes",
        "plaintext",
        "noscript",
        "svg",
        "math",
    ]
    .iter()
    .any(|protected| name.eq_ignore_ascii_case(protected))
    {
        return None;
    }
    let mut output = source[..index].to_string();
    let mut unquoted_value = false;
    loop {
        let separator = index;
        index = skip_space(bytes, index);
        match *bytes.get(index)? {
            b'>' => {
                output.push('>');
                return Some((index + 1, output, is_block));
            }
            b'/' if !closing && bytes.get(index + 1) == Some(&b'>') => {
                if unquoted_value {
                    // Without this separator, '/' becomes part of the unquoted value.
                    output.push(' ');
                }
                output.push_str("/>");
                return Some((index + 2, output, is_block));
            }
            _ if closing || index == separator => return None,
            _ => {}
        }

        let attr_start = index;
        while let Some(&byte) = bytes.get(index) {
            if is_space(byte) || matches!(byte, b'=' | b'/' | b'>') {
                break;
            }
            if matches!(byte, 0 | b'"' | b'\'' | b'<') {
                return None;
            }
            index += 1;
        }
        if attr_start == index {
            return None;
        }
        if source[attr_start..index].eq_ignore_ascii_case("hidden") {
            // A hidden element does not establish a rendered block boundary.
            return None;
        }
        output.push(' ');
        output.push_str(&source[attr_start..index]);
        unquoted_value = false;

        let equals = skip_space(bytes, index);
        if bytes.get(equals) != Some(&b'=') {
            continue;
        }
        output.push('=');
        index = skip_space(bytes, equals + 1);
        let value_start = index;
        match *bytes.get(index)? {
            quote @ (b'"' | b'\'') => {
                index += 1;
                while *bytes.get(index)? != quote {
                    if bytes[index] == 0 {
                        return None;
                    }
                    index += 1;
                }
                index += 1;
            }
            _ => {
                while let Some(&byte) = bytes.get(index) {
                    if is_space(byte) || byte == b'>' {
                        break;
                    }
                    if matches!(byte, 0 | b'"' | b'\'' | b'<' | b'=' | b'`') {
                        return None;
                    }
                    index += 1;
                }
                if index == value_start {
                    return None;
                }
                unquoted_value = true;
            }
        }
        output.push_str(&source[value_start..index]);
    }
}
