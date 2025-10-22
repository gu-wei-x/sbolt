#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::compiler::optimizer;

#[test]
fn optimize_with_default_optimizer() {
    let source = "<html>\n  </html>";
    let options = CompilerOptions::default().with_optimization(true);
    let optimizer = optimizer::create_optimizer(crate::types::template::Kind::KTEXT, &options);
    let output = optimizer.optimize(source);
    assert_eq!(output, source)
}
