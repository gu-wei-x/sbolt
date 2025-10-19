#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::types::Block;
use crate::codegen::types::Template;
use crate::types::result;
use crate::types::template::Kind;

#[test]
#[should_panic]
fn template_from_empty() {
    let options = CompilerOptions::default();
    Template::from("", None, Kind::KHTML, &options).unwrap();
}

#[test]
fn template_from_content() -> result::Result<()> {
    let content = "Hello, world!";
    let options = CompilerOptions::default();
    let template = Template::from(
        &content,
        Some(String::from("testns")),
        Kind::KHTML,
        &options,
    )?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    assert_eq!(block.location().line, 0);
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);
    let first_block = &root_span.blocks()[0];
    assert!(matches!(first_block, Block::KCONTENT(_)));
    assert_eq!(first_block.content(), content);
    assert_eq!(template.namespace(), Some(&String::from("testns")));

    Ok(())
}

#[test]
fn template_from_code() -> result::Result<()> {
    let code = "let x = 10;";
    let content = &format!("@{{{}}}", code);
    let options = CompilerOptions::default();
    let template = Template::from(&content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 1);
            let first_block = &span.blocks()[0];
            assert!(matches!(first_block, Block::KCODE(_)));
            assert_eq!(first_block.content(), code);
        }
        _ => panic!("Expected KROOT block"),
    }

    Ok(())
}

#[test]
fn template_from_inline_code_in_content() -> result::Result<()> {
    let pre_content = "Hello!";
    let code = "name";
    let post_content = "!";

    let content = &format!("{}@{}{}", pre_content, code, post_content);
    let options = CompilerOptions::default();
    let template = Template::from(&content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 3);
            let block = &span.blocks()[0];
            assert!(matches!(block, Block::KCONTENT(_)));
            assert_eq!(block.content(), pre_content);

            let block = &span.blocks()[1];
            assert!(matches!(block, Block::KINLINEDCODE(_)));
            assert_eq!(block.content(), code);

            let block = &span.blocks()[2];
            assert!(matches!(block, Block::KCONTENT(_)));
            assert_eq!(block.content(), post_content);
        }
        _ => panic!("Expected KROOT block"),
    }

    Ok(())
}

#[test]
fn template_from_inlined_code_in_content_within_parentheses() -> result::Result<()> {
    let pre_content = "Hello!";
    let code = "name";
    let post_content = "!";

    let content = &format!("{}@({}){}", pre_content, code, post_content);
    let options = CompilerOptions::default();
    let template = Template::from(&content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 3);
            let block = &span.blocks()[0];
            assert!(matches!(block, Block::KCONTENT(_)));
            assert_eq!(block.content(), pre_content);

            let block = &span.blocks()[1];
            assert!(matches!(block, Block::KINLINEDCODE(_)));
            assert_eq!(block.content(), code);

            let block = &span.blocks()[2];
            assert!(matches!(block, Block::KCONTENT(_)));
            assert_eq!(block.content(), post_content);
        }
        _ => panic!("Expected KROOT block"),
    }

    Ok(())
}

#[test]
fn template_from_code_block_in_content() -> result::Result<()> {
    let pre_content = "Hello!";
    let code = "name";
    let post_content = "!";

    let content = &format!("{}@{{{}}}{}", pre_content, code, post_content);
    let options = CompilerOptions::default();
    let template = Template::from(&content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 3);
            let block = &span.blocks()[0];
            assert!(matches!(block, Block::KCONTENT(_)));
            assert_eq!(block.content(), pre_content);

            let block = &span.blocks()[1];
            assert!(matches!(block, Block::KCODE(_)));
            assert_eq!(block.content(), code);

            let block = &span.blocks()[2];
            assert!(matches!(block, Block::KCONTENT(_)));
            assert_eq!(block.content(), post_content);
        }
        _ => panic!("Expected KROOT block"),
    }

    Ok(())
}

#[test]
fn template_from_inlined_content_in_code_separated_by_lf() -> result::Result<()> {
    let pre_code = "let name = '';";
    let content = "test";
    let post_code = "println!(\"Hello, {}!\", name);";

    let raw_content = &format!("@{{{}@{}\n{}}}", pre_code, content, post_code);
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 1);
            let code_block = &span.blocks()[0];
            assert!(matches!(code_block, Block::KCODE(_)));
            match code_block {
                Block::KCODE(code_span) => {
                    assert_eq!(code_span.blocks().len(), 3);
                    let block = &code_span.blocks()[0];
                    assert!(matches!(block, Block::KCODE(_)));
                    assert_eq!(block.content(), pre_code);

                    let block = &code_span.blocks()[1];
                    assert!(matches!(block, Block::KINLINEDCONTENT(_)));
                    assert_eq!(block.content(), content);
                    assert_eq!(block.location().line, 0);

                    let block = &code_span.blocks()[2];
                    assert!(matches!(block, Block::KCODE(_)));
                    assert_eq!(block.content(), post_code);
                    assert_eq!(block.location().line, 1);
                }
                _ => panic!("Expected KCODE block"),
            }
        }
        _ => panic!("Expected KROOT block"),
    }
    Ok(())
}

#[test]
fn template_from_inlined_content_in_code_separated_by_space() -> result::Result<()> {
    let pre_code = "let name = '';";
    let content = "test";
    let post_code = " println!(\"Hello, {}!\", name);";

    let raw_content = &format!("@{{{}@{}{}}}", pre_code, content, post_code);
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 1);
            let code_block = &span.blocks()[0];
            assert!(matches!(code_block, Block::KCODE(_)));
            match code_block {
                Block::KCODE(code_span) => {
                    assert_eq!(code_span.blocks().len(), 3);
                    let block = &code_span.blocks()[0];
                    assert!(matches!(block, Block::KCODE(_)));
                    assert_eq!(block.content(), pre_code);

                    let block = &code_span.blocks()[1];
                    assert!(matches!(block, Block::KINLINEDCONTENT(_)));
                    assert_eq!(block.content(), content);

                    let block = &code_span.blocks()[2];
                    assert!(matches!(block, Block::KCODE(_)));
                    assert_eq!(block.content(), post_code);
                }
                _ => panic!("Expected KCODE block"),
            }
        }
        _ => panic!("Expected KROOT block"),
    }
    Ok(())
}

#[test]
fn template_from_content_block_in_code() -> result::Result<()> {
    let pre_code = "let name = '';";
    let content = "test";
    let post_code = "println!(\"Hello, {}!\", name);";

    let raw_content = &format!("@{{{}@{{{}}}{}}}", pre_code, content, post_code);
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 1);
            let code_block = &span.blocks()[0];
            assert!(matches!(code_block, Block::KCODE(_)));
            match code_block {
                Block::KCODE(code_span) => {
                    assert_eq!(code_span.blocks().len(), 3);
                    let block = &code_span.blocks()[0];
                    assert!(matches!(block, Block::KCODE(_)));
                    assert_eq!(block.content(), pre_code);

                    let block = &code_span.blocks()[1];
                    assert!(matches!(block, Block::KCONTENT(_)));
                    assert_eq!(block.content(), content);

                    let block = &code_span.blocks()[2];
                    assert!(matches!(block, Block::KCODE(_)));
                    assert_eq!(block.content(), post_code);
                }
                _ => panic!("Expected KCODE block"),
            }
        }
        _ => panic!("Expected KROOT block"),
    }
    Ok(())
}

#[test]
fn template_from_inlined_content_in_code_within_by_parentheses() -> result::Result<()> {
    let pre_code = "let name = '';";
    let content = "test";
    let post_code = "println!(\"Hello, {}!\", name);";

    let raw_content = &format!("@{{{}@({}){}}}", pre_code, content, post_code);
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    match block {
        Block::KROOT(span) => {
            assert_eq!(span.blocks().len(), 1);
            let code_block = &span.blocks()[0];
            assert!(matches!(code_block, Block::KCODE(_)));
            match code_block {
                Block::KCODE(code_span) => {
                    assert_eq!(code_span.blocks().len(), 3);
                    let block = &code_span.blocks()[0];
                    assert!(matches!(block, Block::KCODE(_)));
                    assert_eq!(block.content(), pre_code);

                    let block = &code_span.blocks()[1];
                    assert!(matches!(block, Block::KINLINEDCONTENT(_)));
                    assert_eq!(block.content(), content);

                    let block = &code_span.blocks()[2];
                    assert!(matches!(block, Block::KCODE(_)));
                    assert_eq!(block.content(), post_code);
                }
                _ => panic!("Expected KCODE block"),
            }
        }
        _ => panic!("Expected KROOT block"),
    }
    Ok(())
}

#[test]
fn template_from_doc() -> result::Result<()> {
    let raw_content = r#"
@use test::test;
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
</html>"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 10);

    // 0: use
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KUSE(_)));
    assert_eq!(block.content(), "test::test");
    assert_eq!(block.location().line, 1);

    // 1: layout
    let block = &root_span.blocks()[1];
    assert!(matches!(block, Block::KLAYOUT(_)));
    assert_eq!(block.content(), "test::test");
    assert_eq!(block.location().line, 2);

    // 2: code block
    let block = &root_span.blocks()[2];
    assert!(matches!(block, Block::KCODE(_)));

    // 3: content
    let block = &root_span.blocks()[3];
    assert!(matches!(block, Block::KCONTENT(_)));

    // 4: inline code
    let block = &root_span.blocks()[4];
    assert!(matches!(block, Block::KINLINEDCODE(_)));
    assert_eq!(block.content(), "msg");

    // 5: content
    let block = &root_span.blocks()[5];
    assert!(matches!(block, Block::KCONTENT(_)));
    assert_eq!(block.content(), " - from ");

    // 6: inline code with parentheses
    let block = &root_span.blocks()[6];
    assert!(matches!(block, Block::KINLINEDCODE(_)));
    assert_eq!(block.content(), "name");

    // 7: content with parentheses
    let block = &root_span.blocks()[7];
    assert!(matches!(block, Block::KCONTENT(_)));
    assert_eq!(block.content(), "(");

    // 108: inline code with parentheses
    let block = &root_span.blocks()[8];
    assert!(matches!(block, Block::KINLINEDCODE(_)));
    assert_eq!(block.content(), "age");

    // 9: content with parentheses
    let block = &root_span.blocks()[9];
    assert!(matches!(block, Block::KCONTENT(_)));

    Ok(())
}

// TODO: should this be allowed?
#[test]
fn template_from_doc_with_multiple_layouts() -> result::Result<()> {
    let raw_content = r#"
@layout test::test1;
<html>
   <div>Test</div>
   @layout test::test2;
</html>
@layout test::test3;"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 5);

    // 0: layout
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KLAYOUT(_)));
    assert_eq!(block.content(), "test::test1");

    // 1: content.
    let block = &root_span.blocks()[1];
    assert!(matches!(block, Block::KCONTENT(_)));

    // 2: layout
    let block = &root_span.blocks()[2];
    assert!(matches!(block, Block::KLAYOUT(_)));
    assert_eq!(block.content(), "test::test2");

    // 3: content
    let block = &root_span.blocks()[3];
    assert!(matches!(block, Block::KCONTENT(_)));

    // 4: layout
    let block = &root_span.blocks()[4];
    assert!(matches!(block, Block::KLAYOUT(_)));
    assert_eq!(block.content(), "test::test3");

    Ok(())
}

// TODO: should this be allowed?
#[test]
fn template_from_doc_with_multiple_imports() -> result::Result<()> {
    let raw_content = r#"@use test::test1;
@use test::test2;
<html>
   <div>Test</div>
   @use test::test3;
</html>
@use test::test4;"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 6);

    // 0: use
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KUSE(_)));
    assert_eq!(block.content(), "test::test1");

    // 1: use
    let block = &root_span.blocks()[1];
    assert!(matches!(block, Block::KUSE(_)));
    assert_eq!(block.content(), "test::test2");

    // 2: content.
    let block = &root_span.blocks()[2];
    assert!(matches!(block, Block::KCONTENT(_)));

    // 3: use
    let block = &root_span.blocks()[3];
    assert!(matches!(block, Block::KUSE(_)));
    assert_eq!(block.content(), "test::test3");

    // 4: content.
    let block = &root_span.blocks()[4];
    assert!(matches!(block, Block::KCONTENT(_)));

    // 5: use
    let block = &root_span.blocks()[5];
    assert!(matches!(block, Block::KUSE(_)));
    assert_eq!(block.content(), "test::test4");

    Ok(())
}

/*********************************************************************************************/
// TODO: add codeblock, nested code block mixed with content tests
/*********************************************************************************************/
#[test]
fn template_from_doc_with_simple_section() -> result::Result<()> {
    let raw_content = r#"
@section test1 {
   this is test1
}
<html>
   <div>Test</div>
</html>"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 2);

    // 0: section
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KSECTION(_, _)));
    assert_eq!(block.location().line, 1);

    // 1: content
    let block = &root_span.blocks()[1];
    assert!(matches!(block, Block::KCONTENT(_)));
    assert_eq!(block.location().line, 4);
    Ok(())
}

#[test]
fn template_from_doc_content_composit_section() -> result::Result<()> {
    let raw_content = r#"
@section test1 {
   pre
   @{
      let a =3;
   }
   after
}"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 1);

    // 0: section
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KSECTION(_, _)));
    let section_span = match block {
        Block::KSECTION(_name, span) => span,
        _ => panic!("Expected KSECTION block"),
    };
    assert_eq!(section_span.blocks().len(), 3);
    assert!(matches!(&section_span.blocks()[0], Block::KCONTENT(_)));
    assert!(matches!(&section_span.blocks()[1], Block::KCODE(_)));
    assert!(matches!(&section_span.blocks()[2], Block::KCONTENT(_)));
    Ok(())
}

#[test]
fn template_from_doc_with_multiple_sections() -> result::Result<()> {
    let raw_content = r#"@section test1 {
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
}"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 6);

    // 0: section, newline
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KSECTION(_, _)));

    // 1: section, newline
    let block = &root_span.blocks()[1];
    assert!(matches!(block, Block::KSECTION(_, _)));

    // 2: content
    let block = &root_span.blocks()[2];
    assert!(matches!(block, Block::KCONTENT(_)));

    // 3: section, newline
    let block = &root_span.blocks()[3];
    assert!(matches!(block, Block::KSECTION(_, _)));

    // 4: content
    let block = &root_span.blocks()[4];
    assert!(matches!(block, Block::KCONTENT(_)));

    // 5: section
    let block = &root_span.blocks()[5];
    assert!(matches!(block, Block::KSECTION(_, _)));

    Ok(())
}

#[test]
#[should_panic]
fn template_from_doc_with_nested_sections() {
    let raw_content = r#"@section test1 {
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
</html>"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options).unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
}

// comments.
#[test]
fn template_from_doc_with_comment_in_content() -> result::Result<()> {
    let raw_content = r#"
<html>
   <div>Test</div>
    @* This is a comment *@
</html>
"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options)?;
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
    let root_span = block.span();
    assert_eq!(root_span.blocks().len(), 3);

    // 0: content
    let block = &root_span.blocks()[0];
    assert!(matches!(block, Block::KCONTENT(_)));

    // 1: conmment
    let block = &root_span.blocks()[1];
    assert!(matches!(block, Block::KCOMMENT(_)));
    assert_eq!(block.content(), "@* This is a comment *@");

    // 2: content
    let block = &root_span.blocks()[2];
    assert!(matches!(block, Block::KCONTENT(_)));

    Ok(())
}

#[test]
#[should_panic]
fn template_from_doc_with_comment_in_code() {
    let raw_content = r#"
@{
    let x = 10;
    @* This is a comment *@
}"#;
    let options = CompilerOptions::default();
    let template = Template::from(&raw_content, None, Kind::KHTML, &options).unwrap();
    let block = template.block();
    assert!(matches!(block, Block::KROOT(_)));
}
