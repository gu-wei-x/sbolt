use crate::types::error::RuntimeError;
use std::fmt;

// visiblity pub for crate use.
impl RuntimeError {
    pub fn view_not_found(name: &str) -> Self {
        RuntimeError::NotFound(name.to_string(), format!("View '{}' not found", name))
    }

    pub fn layout_not_found(layout: &str, view_name: &str) -> Self {
        RuntimeError::NotFound(
            layout.to_string(),
            format!("Layout: '{layout}' not found for View: {view_name}"),
        )
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::NotFound(name, message) => {
                write!(f, "View '{}' not found: {}", name, message)
            }
        }
    }
}
