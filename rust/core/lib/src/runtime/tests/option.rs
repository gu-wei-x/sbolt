#![cfg(test)]
use crate::types::{DisplayOption, DisplayOptionRef};

#[test]
fn option_str() {
    let content = "hell world!";
    let display_option = DisplayOptionRef::from(&content);
    let fmt_str = display_option.to_string();
    assert_eq!(fmt_str, content);
}

#[test]
fn option_str_none() {
    let display_option: DisplayOptionRef<'_, &'static str> = DisplayOptionRef(None);
    let fmt_str = display_option.to_string();
    assert_eq!(fmt_str, "");
}

#[test]
fn option_string() {
    let content = String::from("hell world!");
    let display_option = DisplayOption::from(content.clone());
    let fmt_str = display_option.to_string();
    assert_eq!(fmt_str, content);
}

#[test]
fn option_string_none() {
    let display_option: DisplayOption<String> = DisplayOption(None);
    let fmt_str = display_option.to_string();
    assert_eq!(fmt_str, "");
}
