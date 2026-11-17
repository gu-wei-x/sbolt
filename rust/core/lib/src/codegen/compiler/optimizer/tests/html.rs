#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::compiler::optimizer;

#[test]
fn optimize_with_no_optimization() {
    let source = "<html>\n  </html>";
    let options = CompilerOptions::default();
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    let output = optimizer.optimize(source);
    assert_eq!(output, source)
}

#[test]
fn optimize_preserves_empty_non_void_elements() {
    let source = "<html>\n  </html>";
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    let output = optimizer.optimize(source);
    let expected = "<html></html>";
    assert_eq!(output, expected)
}

#[test]
fn optimize_ignore_spaces_and_lf() {
    let source = "<html>\n  test\n  </html>";
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    let output = optimizer.optimize(source);
    let expected = "<html>test</html>";
    assert_eq!(output, expected)
}

#[test]
fn optimize_preserves_empty_element_attributes() {
    let source = r#"
        <div class="c1 c2">
        </div>
    "#;
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    let output = optimizer.optimize(source);
    assert_eq!(output, "<div class=\"c1 c2\"></div>")
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
        assert_eq!(optimizer.optimize(source), source);
    }
    assert_eq!(optimizer.optimize("<BR><p>after</p>"), "<BR/><p>after</p>");
}

#[test]
fn optimize_escapes_attribute_quotes_without_double_escaping_entities() {
    let source = r#"<div title='say "hi" &amp; goodbye'>hello</div>"#;
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    assert_eq!(
        optimizer.optimize(source),
        r#"<div title="say &quot;hi&quot; &amp; goodbye">hello</div>"#
    );
}
