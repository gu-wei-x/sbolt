use crate::types::error::RuntimeError;
use std::fmt;

// visiblity pub for crate use.
impl RuntimeError {
    pub fn view_kind_not_found(name: &str, kind: crate::types::template::Kind) -> Self {
        RuntimeError::NotFound(
            name.to_string(),
            format!("View '{name}' not found for kind {kind:?}"),
        )
    }

    pub fn view_not_found(name: &str) -> Self {
        RuntimeError::NotFound(name.to_string(), format!("View '{}' not found", name))
    }

    pub fn layout_not_found(layout: &str, view_name: &str) -> Self {
        RuntimeError::NotFound(
            view_name.to_string(),
            format!("Layout '{layout}' not found for View `{view_name}`"),
        )
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::NotFound(name, message) => {
                write!(f, "View:{}, NotFound: {}", name, message)
            }
            RuntimeError::AmbiguousView(name) => {
                write!(
                    f,
                    "View '{name}' has multiple formats; pass a Kind to render to select a representation"
                )
            }
            RuntimeError::InvalidInput(summary, detail) => {
                write!(f, "Invalid input: {summary}, {detail}")
            }
        }
    }
}

impl std::error::Error for RuntimeError {}
