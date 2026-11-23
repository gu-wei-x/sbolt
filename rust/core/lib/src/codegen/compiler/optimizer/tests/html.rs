#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::compiler::optimizer;

#[test]
fn optimize_with_no_optimization() {
    let source = "<html>\n  </html>";
    let options = CompilerOptions::default().with_optimization(false);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    let output = optimizer.optimize(source).unwrap();
    assert_eq!(output, source)
}

#[test]
fn optimize_preserves_empty_non_void_elements() {
    let source = "<html>\n  </html>";
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    let output = optimizer.optimize(source).unwrap();
    let expected = "<html></html>";
    assert_eq!(output, expected)
}

#[test]
fn optimize_preserves_text_spaces_and_lf() {
    let source = "<html>\n  test\n  </html>";
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    let output = optimizer.optimize(source).unwrap();
    let expected = source;
    assert_eq!(output, expected)
}

#[test]
fn optimize_preserves_empty_element_attributes() {
    let source = "<div class=\"c1 c2\">\n        </div>";
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    let output = optimizer.optimize(source).unwrap();
    assert_eq!(output, "<div class=\"c1 c2\"></div>")
}

#[test]
fn optimize_removes_formatting_between_block_tags() {
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    for (source, expected) in [
        ("<html>\n  </html>", "<html></html>"),
        (
            "<div>\n  <p>one</p>\n  <p>two</p>\n</div>",
            "<div><p>one</p><p>two</p></div>",
        ),
        (
            "<DIV>\r\n\t<span>one</span>\n</DIV>\n<p>two</p>",
            "<DIV><span>one</span></DIV><p>two</p>",
        ),
        (
            "<!DOCTYPE html>\n<html>\n<head>\n</head>\n<body>\n</body>\n</html>",
            "<!DOCTYPE html><html><head></head><body></body></html>",
        ),
        ("\n <div>\n  </div>\n ", "\n <div></div>\n "),
    ] {
        assert_eq!(optimizer.optimize(source).unwrap(), expected);
    }
}

#[test]
fn optimize_keeps_inline_word_separators() {
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    for (source, expected) in [
        (
            "<span>Hello</span>\n   <span>world</span>",
            "<span>Hello</span> <span>world</span>",
        ),
        (
            "<span>Hello</span><span>world</span>",
            "<span>Hello</span><span>world</span>",
        ),
        (
            "<my-widget>one</my-widget>\n\t<my-widget>two</my-widget>",
            "<my-widget>one</my-widget> <my-widget>two</my-widget>",
        ),
        ("<span>\n  </span>", "<span> </span>"),
        (
            "<div>hello  <span>world</span>  again</div>",
            "<div>hello  <span>world</span>  again</div>",
        ),
        ("<div>&nbsp;\n</div>", "<div>&nbsp;\n</div>"),
        ("<div>\u{a0}\n</div>", "<div>\u{a0}\n</div>"),
        ("<div>\u{000b}\n</div>", "<div>\u{000b}\n</div>"),
        ("<span>one</span>\n ", "<span>one</span>\n "),
    ] {
        assert_eq!(optimizer.optimize(source).unwrap(), expected);
    }
}

#[test]
fn optimize_preserves_separators_around_hidden_elements() {
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    for source in [
        "<span>Hello</span>\n<div hidden></div><span>world</span>",
        "<span>Hello</span><div hidden></div>\n<span>world</span>",
        "<span>Hello</span>\n<DIV id='x' HiDdEn = 'false'></DIV><span>world</span>",
        "<span>Hello</span>\n<div hidden=\"until-found\"></div><span>world</span>",
        "<span>Hello</span>\n<dialog>hidden</dialog><span>world</span>",
        "<span>Hello</span>\n<dialog open>visible</dialog><span>world</span>",
    ] {
        assert_eq!(optimizer.optimize(source).unwrap(), source);
    }

    assert_eq!(
        optimizer
            .optimize("<div   title = 'hidden'   data-hidden = x >text</div>")
            .unwrap(),
        "<div title='hidden' data-hidden=x>text</div>"
    );
}

#[test]
fn optimize_preserves_listing_content() {
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    for source in [
        "<listing><span>one</span>\n   <span>two</span></listing>",
        "<LISTING><span>one</span>\r\n\t<span>two</span></LISTING><div   id='x'></div>",
    ] {
        assert_eq!(optimizer.optimize(source).unwrap(), source);
    }
}

#[test]
fn optimize_preserves_siblings_after_empty_elements() {
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    for source in [
        "<div></div><p>after</p>",
        "<script></script><p>after</p>",
        "<my-widget></my-widget><p>after</p>",
    ] {
        assert_eq!(optimizer.optimize(source).unwrap(), source);
    }
    assert_eq!(
        optimizer.optimize("<BR><p>after</p>").unwrap(),
        "<BR><p>after</p>"
    );
}

#[test]
fn optimize_preserves_attribute_quotes_and_entities() {
    let source = r#"<div title='say "hi" &amp; goodbye'>hello</div>"#;
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    assert_eq!(optimizer.optimize(source).unwrap(), source);
}

#[test]
fn optimize_compacts_only_tag_syntax() {
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    for (source, expected) in [
        (
            "<div \n  class \t= ' a  b '  data-x=\"\u{e9} &amp; &#32;\"   > hi  there </div   >",
            "<div class=' a  b ' data-x=\"\u{e9} &amp; &#32;\"> hi  there </div>",
        ),
        (
            "<input   disabled   value = x />",
            "<input disabled value=x />",
        ),
        ("<img  src = x/>", "<img src=x/>"),
        ("<br  />", "<br/>"),
        (
            "<!DOCTYPE html><my-widget   data-x='y'  ></my-widget>",
            "<!DOCTYPE html><my-widget data-x='y'></my-widget>",
        ),
    ] {
        assert_eq!(optimizer.optimize(source).unwrap(), expected);
    }
}

#[test]
fn optimize_preserves_protected_and_unsupported_regions() {
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    for source in [
        "<div title=\"unfinished",
        "<div a='x'y='z'>text</div>",
        "<!-- application marker --><div   class='x'>text</div>",
        "<pre><span   class='x'> a  b\n</span></pre>",
        "<TEXTAREA><div   class='x'> a  b </div></TEXTAREA>",
        "<script>const text = '<div   class=\"x\">';</script>",
        "<style>div::before { content: '<div   class=\"x\">'; }</style>",
        "<svg><foreignObject><div   class='x'></div></foreignObject></svg>",
        "<math><mtext><span   class='x'> a  b </span></mtext></math>",
        "<title> a  b </title>",
        "<div a=\u{000b}x>",
    ] {
        assert_eq!(optimizer.optimize(source).unwrap(), source);
    }
}
