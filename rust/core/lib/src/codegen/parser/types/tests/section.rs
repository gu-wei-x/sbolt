#![cfg(test)]
use crate::codegen::types::Block;
use crate::codegen::types::Template;
use crate::types::result;
use crate::types::template::Kind;

#[test]
#[should_panic]
fn to_content_panic() {
    let raw_content = r#"
<html>
   <div>Test</div>
</html>"#;
    let template = Template::from(&raw_content, None, Kind::KHTML).unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
    assert_eq!(root_span.blocks().len(), 1);

    // 0: section
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KCONTENT(_)));
    block.to_content();
}

#[test]
fn to_content() -> result::Result<()> {
    let raw_content = r#"
@section test1 {
   this is test1
}"#;
    let template = Template::from(&raw_content, None, Kind::KHTML)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = match block {
        Block::KROOT(span) => span,
        _ => panic!("Expected KROOT block"),
    };
    assert_eq!(root_span.blocks().len(), 1);
    assert_eq!(block.content().trim(), "");

    // 0: section
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KSECTION(_, _)));
    assert_eq!(block.content().trim(), "this is test1");
    let content_block = block.to_content();
    assert!(matches!(content_block, Block::KCONTENT(_)));
    assert_eq!(content_block.content().trim(), "this is test1");

    Ok(())
}
