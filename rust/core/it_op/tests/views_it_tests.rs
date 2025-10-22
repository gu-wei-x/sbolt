#![cfg(test)]
use sbolt::types::result;
sbolt::include_views!();

#[test]
fn comp_index_view() -> result::RenderResult<()> {
    let mut context = sbolt::context! {
        name: "sbolt".to_string(),
        age: 1,
        msg: "Welcome!".to_string()
    };
    let result = lib_it_op_views::render("views/sub/index", &mut context)?;
    let expected = "<html><head><title>Welcome</title></head><body><div>Welcome! - from sbolt(1)</div></body></html>";
    assert_eq!(result, expected);

    Ok(())
}
