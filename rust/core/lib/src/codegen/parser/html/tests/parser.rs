#![cfg(test)]
use crate::codegen::parser::html::parser;

#[test]
fn process_wellformed_node_with_no_meaningful_content() {
    let source = "<div>\n  </div>";
    let mut state_machine = parser::StateMachine::new(source);
    let dom = state_machine.process();
    let content = dom.to_string();
    let expected = "<div/>";
    assert_eq!(content, expected);
}

#[test]
fn process_wellformed_node_with_attrs() {
    let source = r#"
        <div a1="a1" a2="a2">
           test
        </div>
    "#;
    let mut state_machine = parser::StateMachine::new(source);
    let dom = state_machine.process();
    let content = dom.to_string();
    let expected = r#"<div a1="a1" a2="a2">test</div>"#;
    assert_eq!(content, expected);
}
