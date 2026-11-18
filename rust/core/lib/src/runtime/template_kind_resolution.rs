//! Framework-independent selection of a template kind from an HTTP `Accept` value.

use crate::types::{error::RuntimeError, result::RenderResult, template::Kind};
use std::cmp::Reverse;

/// Selects HTML, JSON, or plain text using quality, specificity, and header order.
///
/// A missing header defaults to HTML. Wildcard ties prefer HTML, then JSON, then
/// text. Only UTF-8 charset parameters are supported; extensions after `q` are ignored.
/// The caller remains responsible for extracting headers and setting the response
/// content type and `Vary: Accept`.
///
/// Returns `RuntimeError::InvalidInput` for malformed preferences and
/// `RuntimeError::NotFound` when no supported kind matches.
pub fn resolve_kind(preferences: Option<&str>) -> RenderResult<Kind> {
    let Some(preferences) = preferences else {
        return Ok(Kind::KHTML);
    };
    let mut quoted = false;
    let mut escaped = false;
    let ranges = preferences
        .split(|c| {
            if escaped {
                escaped = false;
            } else if quoted && c == '\\' {
                escaped = true;
            } else if c == '"' {
                quoted = !quoted;
            } else if c == ',' && !quoted {
                return true;
            }
            false
        })
        .map(|range| range.trim().parse::<mime::Mime>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            RuntimeError::InvalidInput(
                "Invalid content preferences".to_string(),
                format!("Could not parse '{preferences}': {error}"),
            )
        })?;

    let mut selected = None;
    for (media, kind) in [
        (mime::TEXT_HTML, Kind::KHTML),
        (mime::APPLICATION_JSON, Kind::KJSON),
        (mime::TEXT_PLAIN, Kind::KTEXT),
    ] {
        let mut matching = None;
        for (index, range) in ranges.iter().enumerate() {
            let mut parameter_count = 0;
            let mut supported_parameters = true;
            let mut quality = 1.0_f32;
            for (name, value) in range.params() {
                if name.as_str() == "q" {
                    quality = value.as_str().parse().map_err(|error| {
                        RuntimeError::InvalidInput(
                            "Invalid quality weight".to_string(),
                            format!(
                                "Could not parse quality '{value}' in '{preferences}': {error}"
                            ),
                        )
                    })?;
                    break;
                }
                parameter_count += 1;
                if name != mime::CHARSET || !value.as_str().eq_ignore_ascii_case("utf-8") {
                    supported_parameters = false;
                }
            }
            if !(0.0..=1.0).contains(&quality) {
                return Err(RuntimeError::InvalidInput(
                    "Invalid quality weight".to_string(),
                    format!("Quality {quality} in '{preferences}' must be between 0 and 1"),
                ));
            }
            if !supported_parameters
                || (range.type_() != mime::STAR && range.type_() != media.type_())
                || (range.subtype() != mime::STAR
                    && (range.subtype() != media.subtype() || range.suffix() != media.suffix()))
            {
                continue;
            }
            let specificity = (
                u8::from(range.type_() != mime::STAR) + u8::from(range.subtype() != mime::STAR),
                parameter_count,
            );
            let priority = (specificity, Reverse(index));
            if matching.is_none_or(|(current, _)| priority > current) {
                matching = Some((priority, quality));
            }
        }
        if let Some(((specificity, order), quality)) = matching {
            if quality > 0.0 {
                let score = (quality, specificity, order);
                if selected.is_none_or(|(_, current)| score > current) {
                    selected = Some((kind, score));
                }
            }
        }
    }
    selected.map(|(kind, _)| kind).ok_or_else(|| {
        RuntimeError::NotFound(
            "Template kind".to_string(),
            format!("No HTML, JSON, or plain-text kind matches '{preferences}'"),
        )
    })
}
