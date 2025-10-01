#![cfg(test)]
use crate::{
    codegen::{
        consts,
        parser::template::{self, block},
    },
    types::error,
};

#[test]
#[should_panic]
fn test_template_from_empty() {
    template::Template::from("", None).unwrap();
}

#[test]
fn test_template_from_content() -> core::result::Result<(), error::Error> {
    let content = "Hello, world!";
    let template = template::Template::from(&content, None)?;
    let block = template.block();
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 1);

    let content_block = &block.blocks()[0];
    assert_eq!(content_block.name(), None);
    assert_eq!(content_block.kind(), block::Kind::CONTENT);
    assert_eq!(content_block.content(), content);
    assert_eq!(content_block.has_blocks(), false);
    Ok(())
}

#[test]
fn test_template_from_code() -> core::result::Result<(), error::Error> {
    let code = "let x = 10;";
    let content = &format!("@{{{}}}", code);
    let template = template::Template::from(&content, None)?;
    let block = template.block();
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 1);

    let content_block = &block.blocks()[0];
    assert_eq!(content_block.name(), None);
    assert_eq!(content_block.kind(), block::Kind::CODE);
    assert_eq!(content_block.content(), code);
    assert_eq!(content_block.has_blocks(), false);
    Ok(())
}

#[test]
fn test_template_from_inline_code_in_content() -> core::result::Result<(), error::Error> {
    let pre_content = "Hello!";
    let code = "name";
    let post_content = "!";

    let content = &format!("{}@{}{}", pre_content, code, post_content);
    let template = template::Template::from(&content, None)?;
    let block = template.block();
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 3);

    let blocks = block.blocks();
    assert_eq!(blocks[0].kind(), template::block::Kind::CONTENT);
    assert_eq!(blocks[0].content(), pre_content);
    assert_eq!(blocks[1].kind(), template::block::Kind::INLINEDCODE);
    assert_eq!(blocks[1].content(), code);
    assert_eq!(blocks[2].kind(), template::block::Kind::CONTENT);
    assert_eq!(blocks[2].content(), post_content);
    Ok(())
}

#[test]
fn test_template_from_inlined_code_in_content2() -> core::result::Result<(), error::Error> {
    let pre_content = "Hello!";
    let code = "name";
    let post_content = "!";

    let content = &format!("{}@({}){}", pre_content, code, post_content);
    let template = template::Template::from(&content, None)?;
    let block = template.block();
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 3);

    let blocks = block.blocks();
    assert_eq!(blocks[0].kind(), template::block::Kind::CONTENT);
    assert_eq!(blocks[0].content(), pre_content);
    assert_eq!(blocks[1].kind(), template::block::Kind::INLINEDCODE);
    assert_eq!(blocks[1].content(), code);
    assert_eq!(blocks[2].kind(), template::block::Kind::CONTENT);
    assert_eq!(blocks[2].content(), post_content);
    Ok(())
}

#[test]
fn test_template_from_code_block_in_content() -> core::result::Result<(), error::Error> {
    let pre_content = "Hello!";
    let code = "name";
    let post_content = "!";

    let content = &format!("{}@{{{}}}{}", pre_content, code, post_content);
    let template = template::Template::from(&content, None)?;
    let block = template.block();
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 3);

    let blocks = block.blocks();
    assert_eq!(blocks[0].kind(), template::block::Kind::CONTENT);
    assert_eq!(blocks[0].content(), pre_content);
    assert_eq!(blocks[1].kind(), template::block::Kind::CODE);
    assert_eq!(blocks[1].content(), code);
    assert_eq!(blocks[2].kind(), template::block::Kind::CONTENT);
    assert_eq!(blocks[2].content(), post_content);
    Ok(())
}

#[test]
fn test_template_from_inlined_content_in_code() -> core::result::Result<(), error::Error> {
    let pre_code = "let name = '';";
    let content = "test";
    let post_code = "\nprintln!(\"Hello, {}!\", name);";

    let raw_content = &format!("@{{{}@{}{}}}", pre_code, content, post_code);
    let template = template::Template::from(&raw_content, None)?;
    let block = template.block();
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 1);

    let code_block = &block.blocks()[0];
    assert_eq!(code_block.name(), None);
    assert_eq!(code_block.kind(), template::block::Kind::CODE);
    assert!(code_block.has_blocks());
    assert_eq!(code_block.blocks().len(), 3);

    let blocks = code_block.blocks();
    assert_eq!(blocks[0].kind(), template::block::Kind::CODE);
    assert_eq!(blocks[0].content(), pre_code);
    assert_eq!(blocks[1].kind(), template::block::Kind::INLINEDCONTENT);
    assert_eq!(blocks[1].content(), content);
    assert_eq!(blocks[2].kind(), template::block::Kind::CODE);
    assert_eq!(blocks[2].content(), post_code);
    Ok(())
}

#[test]
fn test_template_from_inlined_content_in_code2() -> core::result::Result<(), error::Error> {
    let pre_code = "let name = '';";
    let content = "test";
    let post_code = " println!(\"Hello, {}!\", name);";

    let raw_content = &format!("@{{{}@{}{}}}", pre_code, content, post_code);
    let template = template::Template::from(&raw_content, None)?;
    let block = template.block();
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 1);

    let code_block = &block.blocks()[0];
    assert_eq!(code_block.name(), None);
    assert_eq!(code_block.kind(), template::block::Kind::CODE);
    assert!(code_block.has_blocks());
    assert_eq!(code_block.blocks().len(), 3);

    let blocks = code_block.blocks();
    assert_eq!(blocks[0].kind(), template::block::Kind::CODE);
    assert_eq!(blocks[0].content(), pre_code);
    assert_eq!(blocks[1].kind(), template::block::Kind::INLINEDCONTENT);
    assert_eq!(blocks[1].content(), content);
    assert_eq!(blocks[2].kind(), template::block::Kind::CODE);
    assert_eq!(blocks[2].content(), post_code);
    Ok(())
}

#[test]
fn test_template_from_content_block_in_code() -> core::result::Result<(), error::Error> {
    let pre_code = "let name = '';";
    let content = "test";
    let post_code = "println!(\"Hello, {}!\", name);";

    let raw_content = &format!("@{{{}@{{{}}}{}}}", pre_code, content, post_code);
    let template = template::Template::from(&raw_content, None)?;
    let block = template.block();
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 1);

    let code_block = &block.blocks()[0];
    assert_eq!(code_block.name(), None);
    assert_eq!(code_block.kind(), template::block::Kind::CODE);
    assert!(code_block.has_blocks());
    assert_eq!(code_block.blocks().len(), 3);

    let blocks = code_block.blocks();
    assert_eq!(blocks[0].kind(), template::block::Kind::CODE);
    assert_eq!(blocks[0].content(), pre_code);
    assert_eq!(blocks[1].kind(), template::block::Kind::CONTENT);
    assert_eq!(blocks[1].content(), content);
    assert_eq!(blocks[2].kind(), template::block::Kind::CODE);
    assert_eq!(blocks[2].content(), post_code);
    Ok(())
}

#[test]
fn test_template_from_lined_content_in_code() -> core::result::Result<(), error::Error> {
    let pre_code = "let name = '';";
    let content = "test";
    let post_code = "println!(\"Hello, {}!\", name);";

    let raw_content = &format!("@{{{}@({}){}}}", pre_code, content, post_code);
    let template = template::Template::from(&raw_content, None)?;
    let block = template.block();
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 1);

    let code_block = &block.blocks()[0];
    assert_eq!(code_block.name(), None);
    assert_eq!(code_block.kind(), template::block::Kind::CODE);
    assert!(code_block.has_blocks());
    assert_eq!(code_block.blocks().len(), 3);

    let blocks = code_block.blocks();
    assert_eq!(blocks[0].kind(), template::block::Kind::CODE);
    assert_eq!(blocks[0].content(), pre_code);
    assert_eq!(blocks[1].kind(), template::block::Kind::INLINEDCONTENT);
    assert_eq!(blocks[1].content(), content);
    assert_eq!(blocks[2].kind(), template::block::Kind::CODE);
    assert_eq!(blocks[2].content(), post_code);
    Ok(())
}

#[test]
fn test_template_from_doc() -> core::result::Result<(), error::Error> {
    let raw_content = r#"
@layout test::test;
@{
    let msg = "Hello";
    let name = "Test";
    let age = 30;
}
<html>
    <head>
        <title>test</title>
    </head>
    <body>
        <div>@msg - from @name(@age)</div>
    </body>
</html>    
    "#;
    let template = template::Template::from(&raw_content, None)?;
    let block = template.block();

    // root
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 9);

    // 1st block - layout directive.
    assert_eq!(block.blocks()[0].kind(), template::block::Kind::DIRECTIVE);
    assert_eq!(block.blocks()[0].content(), "test::test");
    assert_eq!(
        block.blocks()[0].name(),
        Some(&consts::DIRECTIVE_KEYWORD_LAYOUT.to_string())
    );

    // 2nd block - code block.
    assert_eq!(block.blocks()[1].kind(), template::block::Kind::CODE);

    // 3rd block - content block
    assert_eq!(block.blocks()[2].kind(), template::block::Kind::CONTENT);

    // 4th block - content block with inlined code and inlined content.
    assert_eq!(block.blocks()[3].kind(), template::block::Kind::INLINEDCODE);
    assert_eq!(block.blocks()[3].content(), "msg");

    // 5th block - content block
    assert_eq!(block.blocks()[4].kind(), template::block::Kind::CONTENT);

    // 6th block - content block with inlined code and inlined content.
    assert_eq!(block.blocks()[5].kind(), template::block::Kind::INLINEDCODE);
    assert_eq!(block.blocks()[5].content(), "name");

    // 7th block - content block
    assert_eq!(block.blocks()[6].kind(), template::block::Kind::CONTENT);

    // 8th block - content block with inlined code and inlined content.
    assert_eq!(block.blocks()[7].kind(), template::block::Kind::INLINEDCODE);
    assert_eq!(block.blocks()[7].content(), "age");

    // 9th block - content block
    assert_eq!(block.blocks()[8].kind(), template::block::Kind::CONTENT);
    Ok(())
}

// TODO: should this be allowed?
#[test]
fn test_template_from_doc_with_multiple_layouts() -> core::result::Result<(), error::Error> {
    let raw_content = r#"
@layout test::test1;
<html>
   <div>Test</div>
   @layout test::test2;
</html>
@layout test::test3;"#;
    let template = template::Template::from(&raw_content, None)?;
    let block = template.block();

    // root
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 5);

    // 1st block - layout directive.
    assert_eq!(block.blocks()[0].kind(), template::block::Kind::DIRECTIVE);
    assert_eq!(
        block.blocks()[0].name(),
        Some(&consts::DIRECTIVE_KEYWORD_LAYOUT.to_string())
    );
    assert_eq!(block.blocks()[0].content(), "test::test1");

    // 2nd block - content block
    assert_eq!(block.blocks()[2].kind(), template::block::Kind::DIRECTIVE);
    assert_eq!(
        block.blocks()[2].name(),
        Some(&consts::DIRECTIVE_KEYWORD_LAYOUT.to_string())
    );
    assert_eq!(block.blocks()[2].content(), "test::test2");

    // 3rd block - layout directive.
    assert_eq!(block.blocks()[4].kind(), template::block::Kind::DIRECTIVE);
    assert_eq!(
        block.blocks()[4].name(),
        Some(&consts::DIRECTIVE_KEYWORD_LAYOUT.to_string())
    );
    assert_eq!(block.blocks()[4].content(), "test::test3");
    Ok(())
}

// TODO: should this be allowed?
#[test]
fn test_template_from_doc_with_multiple_imports() -> core::result::Result<(), error::Error> {
    let raw_content = r#"
@use test::test1;
@use test::test2;
<html>
   <div>Test</div>
   @use test::test3;
</html>
@use test::test4;"#;
    let template = template::Template::from(&raw_content, None)?;
    let block = template.block();
    println!("Block: {:#?}", block);

    // root
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 6);

    // 1st block - layout directive.
    assert_eq!(block.blocks()[0].kind(), template::block::Kind::DIRECTIVE);
    assert_eq!(
        block.blocks()[0].name(),
        Some(&consts::DIRECTIVE_KEYWORD_USE.to_string())
    );
    assert_eq!(block.blocks()[0].content(), "test::test1");

    // 2nd block - layout directive.
    assert_eq!(block.blocks()[1].kind(), template::block::Kind::DIRECTIVE);
    assert_eq!(
        block.blocks()[1].name(),
        Some(&consts::DIRECTIVE_KEYWORD_USE.to_string())
    );
    assert_eq!(block.blocks()[1].content(), "test::test2");

    // 4rd block - content block
    assert_eq!(block.blocks()[3].kind(), template::block::Kind::DIRECTIVE);
    assert_eq!(
        block.blocks()[3].name(),
        Some(&consts::DIRECTIVE_KEYWORD_USE.to_string())
    );
    assert_eq!(block.blocks()[3].content(), "test::test3");

    // 6th block - layout directive.
    assert_eq!(block.blocks()[5].kind(), template::block::Kind::DIRECTIVE);
    assert_eq!(
        block.blocks()[5].name(),
        Some(&consts::DIRECTIVE_KEYWORD_USE.to_string())
    );
    assert_eq!(block.blocks()[5].content(), "test::test4");
    Ok(())
}

/*********************************************************************************************/
// TODO: add codeblock, nested code block mixed with content tests
/*********************************************************************************************/
#[test]
fn test_template_from_doc_with_section() -> core::result::Result<(), error::Error> {
    let raw_content = r#"
@section test1 {
   this is test1
}
<html>
   <div>Test</div>
</html>"#;
    let template = template::Template::from(&raw_content, None)?;
    let block = template.block();

    // root
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 2);

    // 1st block - section block.
    assert_eq!(block.blocks()[0].kind(), template::block::Kind::SECTION);
    assert_eq!(block.blocks()[0].name(), Some(&"test1".to_string()));
    assert_eq!(
        block.blocks()[0].content().trim(),
        "this is test1" /* without leading/trailing new lines */
    );

    // 2nd block - content block
    assert_eq!(block.blocks()[1].kind(), template::block::Kind::CONTENT);
    Ok(())
}

#[test]
fn test_template_from_doc_with_multiple_sections() -> core::result::Result<(), error::Error> {
    let raw_content = r#"
@section test1 {
   this is test1
}
@section test2 {
   this is test2
}
<html>
   <div>Test</div>
@section test3 {
   this is test3
}
</html>
@section test4 {
   this is test4
}
"#;
    let template = template::Template::from(&raw_content, None)?;
    let block = template.block();

    // root
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 6);

    Ok(())
}

#[test]
#[should_panic]
fn test_template_from_doc_with_nested_sections() {
    let raw_content = r#"
@section test1 {
   this is test1
   @section test2 {
      this is test2
    }
}
<html>
   <div>Test</div>
@section test3 {
   this is test3
   @section test4 {
      this is test4
   }
}
</html>
"#;
    let template = template::Template::from(&raw_content, None).unwrap();
    let block = template.block();
    // root
    assert_eq!(block.name(), None);
    assert_eq!(block.kind(), block::Kind::ROOT);
    assert!(block.has_blocks());
    assert_eq!(block.blocks().len(), 4);
}
