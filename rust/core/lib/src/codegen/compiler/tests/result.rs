#![cfg(test)]
use crate::codegen::CompileResult;

#[test]
fn compile_result() {
    let mut result = CompileResult::new();
    result.add_warning("test".into());
    let warnings = result.warnings();
    assert!(!warnings.is_empty())
}

#[test]
fn merging_views_rejects_duplicate_kinds_but_accepts_distinct_kinds()
-> crate::types::result::Result<()> {
    use crate::types::template::Kind;
    let mut result = CompileResult::new();
    result.add_view_mapping("view".into(), Kind::KHTML, "view_html::View".into())?;
    let mut json = CompileResult::new();
    json.add_view_mapping("view".into(), Kind::KJSON, "view_json::View".into())?;
    json.merge_into(&mut result)?;
    assert_eq!(result.view_name_mapping().len(), 2);

    let mut duplicate = CompileResult::new();
    duplicate.add_view_mapping("view".into(), Kind::KHTML, "other::View".into())?;
    let error = duplicate.merge_into(&mut result).unwrap_err();
    assert!(error.to_string().contains("Duplicate template 'view'"));
    Ok(())
}
