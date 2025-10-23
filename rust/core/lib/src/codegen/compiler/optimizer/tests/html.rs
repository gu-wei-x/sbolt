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
fn optimize_ignore_close_tag() {
    let source = "<html>\n  </html>";
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    let output = optimizer.optimize(source);
    let expected = "<html/>";
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
fn optimize_ignore_close_tag_3() {
    let source = r#"
        <div class="c1 c2">
        </div>
    "#;
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KHTML, &options);
    let output = optimizer.optimize(source);
    assert_eq!(output, "<div class=\"c1 c2\"/>")
}
