/* private modules implementation of pub types */
mod runtime;

// mod for build.
pub mod codegen;

// mod for build and runtime.
pub mod types;

pub use runtime::template_kind_resolution::resolve_kind;
pub mod utils {
    pub use crate::types::{escape_html, json, raw, raw_json};
}
