use crate::resolve_kind;
use crate::types::error::RuntimeError;
use crate::types::template::Kind;

#[test]
fn selects_by_quality_specificity_and_header_order() {
    assert_eq!(resolve_kind(None).unwrap(), Kind::KHTML);
    for (accept, kind) in [
        ("text/html", Kind::KHTML),
        ("application/json", Kind::KJSON),
        ("text/plain", Kind::KTEXT),
        ("application/json;q=0.9, text/html;q=0.1", Kind::KJSON),
        ("application/xml, application/json;q=0.5", Kind::KJSON),
        ("text/*", Kind::KHTML),
        ("application/*", Kind::KJSON),
        ("*/*", Kind::KHTML),
        ("text/html;q=0, */*;q=0.7", Kind::KJSON),
        ("application/json;q=0, */*;q=0.8", Kind::KHTML),
        ("application/json, text/html", Kind::KJSON),
        ("text/html;level=1, application/json;q=0.5", Kind::KJSON),
        (
            "text/html;charset=utf-8;q=0, application/json;q=0.5",
            Kind::KJSON,
        ),
        (
            "text/html;q=0.8;extension=\"a,b\", application/json;q=0.5",
            Kind::KHTML,
        ),
    ] {
        assert_eq!(resolve_kind(Some(accept)).unwrap(), kind, "{accept}");
    }
}

#[test]
fn rejects_unsupported_and_invalid_accept_headers() {
    for accept in [
        "application/xml",
        "*/*;q=0",
        "text/html;q=0, application/json;q=0, text/plain;q=0, */*;q=1",
        "application/json+other",
    ] {
        assert!(
            matches!(
                resolve_kind(Some(accept)),
                Err(RuntimeError::NotFound(summary, detail))
                    if !summary.is_empty() && detail.contains(accept)
            ),
            "{accept}"
        );
    }
    for accept in [
        "invalid",
        "text/html;q=invalid",
        "application/json;q=1.5",
        "application/xml;q=invalid, application/json",
        "application/xml;q=1.5, application/json",
    ] {
        assert!(
            matches!(
                resolve_kind(Some(accept)),
                Err(RuntimeError::InvalidInput(summary, detail))
                    if !summary.is_empty() && detail.contains(accept)
            ),
            "{accept}"
        );
    }
}
