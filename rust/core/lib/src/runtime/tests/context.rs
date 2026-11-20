#![cfg(test)]
use crate::types::{Context, DefaultViewContext};
use std::sync::Arc;

#[test]
fn context_macro_wraps_a_single_identifier_in_arc_under_model() {
    let value = String::from("model value");
    let context = crate::context!(value);
    assert_eq!(
        context
            .get_data::<Arc<String>>("model")
            .map(|value| value.as_str()),
        Some("model value")
    );
    assert!(context.get_data::<Arc<String>>("value").is_none());
}

#[test]
fn context_macro_evaluates_a_model_expression_once_with_a_trailing_comma() {
    use std::cell::Cell;

    let calls = Cell::new(0);
    let context = crate::context!({
        calls.set(calls.get() + 1);
        String::from("model value")
    },);
    assert_eq!(calls.get(), 1);
    assert_eq!(
        context.get_data::<Arc<String>>("model").unwrap().as_str(),
        "model value"
    );
}

#[test]
fn context_macro_preserves_explicit_keys_and_empty_contexts() {
    let context = crate::context!(title: "page", count: 3usize);
    assert_eq!(context.get_data::<&str>("title"), Some(&"page"));
    assert_eq!(context.get_data::<usize>("count"), Some(&3));
    let context = crate::context!(page: 4usize);
    assert_eq!(context.get_data::<usize>("page"), Some(&4));
    assert!(context.get_data::<usize>("model").is_none());
    let model = Arc::new(String::from("model value"));
    let context = crate::context!(model: Arc::clone(&model));
    assert!(Arc::ptr_eq(
        context.get_data::<Arc<String>>("model").unwrap(),
        &model
    ));
    assert!(crate::context!().get_data::<usize>("model").is_none());
}

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

#[test]
fn default_context_appends_sections_in_order() {
    let mut context = DefaultViewContext::new();
    context.add_section("scripts", "first".into());
    context.add_section("scripts", "second".into());
    assert_eq!(
        context.get_section("scripts").unwrap(),
        &["first", "second"]
    );
}
