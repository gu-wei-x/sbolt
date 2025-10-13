#![cfg(test)]

use crate::types::Template;

struct TestTemplate;

impl Template for TestTemplate {
    fn name() -> String {
        todo!()
    }

    fn kind() -> crate::types::template::Kind {
        todo!()
    }

    fn render(
        &self,
        _context: &mut impl crate::types::Context,
    ) -> crate::types::result::RenderResult<String> {
        todo!()
    }
}

#[test]
fn tempalte_default_layout() {
    let layout = TestTemplate::layout();
    assert!(layout.is_none());
}
