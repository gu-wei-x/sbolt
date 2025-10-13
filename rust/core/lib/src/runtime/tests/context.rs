#![cfg(test)]
use crate::types::{Context, DefaultViewContext};

#[test]
fn default_context_state() {
    let mut context = DefaultViewContext::new();
    context.set_data("key", || "test".to_string());
    let data = context.get_data::<String>("key");
    assert!(data.is_some());
    let data = data.unwrap();
    assert_eq!(data, "test");
}

#[test]
fn default_context_section() {
    let mut context = DefaultViewContext::new();
    context.add_section("s1", String::from("S1"));
    let sections = context.get_section("s1");
    assert!(sections.is_some());
    let sections = sections.unwrap();
    assert!(sections.contains(&String::from("S1")));

    let sections = context.get_section_mut("s1");
    assert!(sections.is_some());
    let sections = sections.unwrap();
    sections.clear();

    let sections = context.get_section("s1");
    assert!(sections.is_some());
    let sections = sections.unwrap();
    assert!(!sections.contains(&String::from("S1")));
}

#[test]
fn default_context_default_section() {
    let mut context = DefaultViewContext::new();
    let default_section_content = context.get_default_section();
    assert!(default_section_content.is_none());

    context.set_default_section(String::from("S1"));
    let default_section_content = context.get_default_section();
    assert!(default_section_content.is_some());
    let default_section_content = default_section_content.unwrap();
    assert_eq!(default_section_content, "S1");

    context.set_default_section(String::from("S2"));
    let default_section_content = context.get_default_section();
    assert!(default_section_content.is_some());
    let default_section_content = default_section_content.unwrap();
    assert_eq!(default_section_content, "S2");
}
