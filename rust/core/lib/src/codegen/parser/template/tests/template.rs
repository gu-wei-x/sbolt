#![cfg(test)]
use crate::{codegen::parser::template, types::error};

#[test]
#[should_panic]
fn test_template_from_empty() {
    template::Template::from("", None).unwrap();
}

#[test]
fn test_template_from_content() -> core::result::Result<(), error::Error> {
    let content = "Hello, world!";
    let template = template::Template::from(&content, None)?;
    assert_eq!(template.blocks.len(), 1);
    assert!(matches!(
        template.blocks[0].span.kind(),
        template::block::Kind::CONTENT(_)
    ));
    assert_eq!(template.blocks[0].content(), content);
    Ok(())
}

#[test]
fn test_template_from_code() -> core::result::Result<(), error::Error> {
    let code = "let x = 10;";
    let content = &format!("@{{{}}}", code);
    let template = template::Template::from(&content, None)?;
    assert_eq!(template.blocks.len(), 1);
    assert!(matches!(
        template.blocks[0].span.kind(),
        template::block::Kind::CODE(_)
    ));
    assert_eq!(template.blocks[0].content(), code);
    Ok(())
}

#[test]
fn test_template_from_inline_code_in_content() -> core::result::Result<(), error::Error> {
    let pre_content = "Hello!";
    let code = "name";
    let post_content = "!";

    let content = &format!("{}@{}{}", pre_content, code, post_content);
    let template = template::Template::from(&content, None)?;
    assert_eq!(template.blocks.len(), 1);
    assert!(matches!(
        template.blocks[0].span.kind(),
        template::block::Kind::CONTENT(_)
    ));

    let block = &template.blocks[0];
    assert_eq!(block.blocks.len(), 3);
    assert_eq!(block.content(), "");

    assert!(matches!(
        block.blocks[0].span.kind(),
        template::block::Kind::CONTENT(_)
    ));
    assert_eq!(block.blocks[0].content(), pre_content);
    assert!(matches!(
        block.blocks[1].span.kind(),
        template::block::Kind::INLINEDCODE(_)
    ));
    assert_eq!(block.blocks[1].content(), code);
    assert!(matches!(
        block.blocks[2].span.kind(),
        template::block::Kind::CONTENT(_)
    ));
    assert_eq!(block.blocks[2].content(), post_content);
    Ok(())
}

#[test]
fn test_template_from_inlined_code_in_content2() -> core::result::Result<(), error::Error> {
    let pre_content = "Hello!";
    let code = "name";
    let post_content = "!";

    let content = &format!("{}@({}){}", pre_content, code, post_content);
    let template = template::Template::from(&content, None)?;
    assert_eq!(template.blocks.len(), 1);
    assert!(matches!(
        template.blocks[0].span.kind(),
        template::block::Kind::CONTENT(_)
    ));

    let block = &template.blocks[0];
    assert_eq!(block.blocks.len(), 3);
    assert_eq!(block.content(), "");

    assert!(matches!(
        block.blocks[0].span.kind(),
        template::block::Kind::CONTENT(_)
    ));
    assert_eq!(block.blocks[0].content(), pre_content);
    assert!(matches!(
        block.blocks[1].span.kind(),
        template::block::Kind::INLINEDCODE(_)
    ));
    assert_eq!(block.blocks[1].content(), code);
    assert!(matches!(
        block.blocks[2].span.kind(),
        template::block::Kind::CONTENT(_)
    ));
    assert_eq!(block.blocks[2].content(), post_content);
    Ok(())
}

#[test]
fn test_template_from_code_block_in_content() -> core::result::Result<(), error::Error> {
    let pre_content = "Hello!";
    let code = "name";
    let post_content = "!";

    let content = &format!("{}@{{{}}}{}", pre_content, code, post_content);
    let template = template::Template::from(&content, None)?;
    assert_eq!(template.blocks.len(), 1);
    assert!(matches!(
        template.blocks[0].span.kind(),
        template::block::Kind::CONTENT(_)
    ));

    let block = &template.blocks[0];
    assert_eq!(block.blocks.len(), 3);
    assert_eq!(block.content(), "");

    assert!(matches!(
        block.blocks[0].span.kind(),
        template::block::Kind::CONTENT(_)
    ));
    assert_eq!(block.blocks[0].content(), pre_content);
    assert!(matches!(
        block.blocks[1].span.kind(),
        template::block::Kind::CODE(_)
    ));
    assert_eq!(block.blocks[1].content(), code);
    assert!(matches!(
        block.blocks[2].span.kind(),
        template::block::Kind::CONTENT(_)
    ));
    assert_eq!(block.blocks[2].content(), post_content);
    Ok(())
}

#[test]
fn test_template_from_inlined_content_in_code() -> core::result::Result<(), error::Error> {
    let pre_code = "let name = '';";
    let content = "test";
    let post_code = "\nprintln!(\"Hello, {}!\", name);";

    let raw_content = &format!("@{{{}@{}{}}}", pre_code, content, post_code);
    let template = template::Template::from(&raw_content, None)?;
    assert_eq!(template.blocks.len(), 1);
    assert!(matches!(
        template.blocks[0].span.kind(),
        template::block::Kind::CODE(_)
    ));

    let block = &template.blocks[0];
    assert_eq!(block.blocks.len(), 3);
    assert_eq!(block.content(), "");

    assert!(matches!(
        block.blocks[0].span.kind(),
        template::block::Kind::CODE(_)
    ));
    assert_eq!(block.blocks[0].content(), pre_code);
    assert!(matches!(
        block.blocks[1].span.kind(),
        template::block::Kind::INLINEDCONTENT(_)
    ));
    assert_eq!(block.blocks[1].content(), content);
    assert!(matches!(
        block.blocks[2].span.kind(),
        template::block::Kind::CODE(_)
    ));
    assert_eq!(block.blocks[2].content(), post_code);
    Ok(())
}

#[test]
fn test_template_from_inlined_content_in_code2() -> core::result::Result<(), error::Error> {
    let pre_code = "let name = '';";
    let content = "test";
    let post_code = " println!(\"Hello, {}!\", name);";

    let raw_content = &format!("@{{{}@{}{}}}", pre_code, content, post_code);
    let template = template::Template::from(&raw_content, None)?;
    assert_eq!(template.blocks.len(), 1);
    assert!(matches!(
        template.blocks[0].span.kind(),
        template::block::Kind::CODE(_)
    ));

    let block = &template.blocks[0];
    assert_eq!(block.blocks.len(), 3);
    assert_eq!(block.content(), "");

    assert!(matches!(
        block.blocks[0].span.kind(),
        template::block::Kind::CODE(_)
    ));
    assert_eq!(block.blocks[0].content(), pre_code);
    assert!(matches!(
        block.blocks[1].span.kind(),
        template::block::Kind::INLINEDCONTENT(_)
    ));
    assert_eq!(block.blocks[1].content(), content);
    assert!(matches!(
        block.blocks[2].span.kind(),
        template::block::Kind::CODE(_)
    ));
    assert_eq!(block.blocks[2].content(), post_code);
    Ok(())
}

#[test]
fn test_template_from_content_block_in_code() -> core::result::Result<(), error::Error> {
    let pre_code = "let name = '';";
    let content = "test";
    let post_code = "println!(\"Hello, {}!\", name);";

    let raw_content = &format!("@{{{}@{{{}}}{}}}", pre_code, content, post_code);
    let template = template::Template::from(&raw_content, None)?;
    assert_eq!(template.blocks.len(), 1);
    assert!(matches!(
        template.blocks[0].span.kind(),
        template::block::Kind::CODE(_)
    ));

    let block = &template.blocks[0];
    assert_eq!(block.blocks.len(), 3);
    assert_eq!(block.content(), "");

    assert!(matches!(
        block.blocks[0].span.kind(),
        template::block::Kind::CODE(_)
    ));
    assert_eq!(block.blocks[0].content(), pre_code);
    assert!(matches!(
        block.blocks[1].span.kind(),
        template::block::Kind::CONTENT(_)
    ));
    assert_eq!(block.blocks[1].content(), content);
    assert!(matches!(
        block.blocks[2].span.kind(),
        template::block::Kind::CODE(_)
    ));
    assert_eq!(block.blocks[2].content(), post_code);
    Ok(())
}

#[test]
fn test_template_from_lined_content_in_code() -> core::result::Result<(), error::Error> {
    let pre_code = "let name = '';";
    let content = "test";
    let post_code = "println!(\"Hello, {}!\", name);";

    let raw_content = &format!("@{{{}@({}){}}}", pre_code, content, post_code);
    let template = template::Template::from(&raw_content, None)?;
    assert_eq!(template.blocks.len(), 1);
    assert!(matches!(
        template.blocks[0].span.kind(),
        template::block::Kind::CODE(_)
    ));

    let block = &template.blocks[0];
    assert_eq!(block.blocks.len(), 3);
    assert_eq!(block.content(), "");

    assert!(matches!(
        block.blocks[0].span.kind(),
        template::block::Kind::CODE(_)
    ));
    assert_eq!(block.blocks[0].content(), pre_code);
    assert!(matches!(
        block.blocks[1].span.kind(),
        template::block::Kind::INLINEDCONTENT(_)
    ));
    assert_eq!(block.blocks[1].content(), content);
    assert!(matches!(
        block.blocks[2].span.kind(),
        template::block::Kind::CODE(_)
    ));
    assert_eq!(block.blocks[2].content(), post_code);
    Ok(())
}
