/* private modules implementation of pub types */
mod runtime;

// mod for build.
pub mod codegen;

// mod for build and runtime.
pub mod types;

pub use runtime::template_kind_resolution::resolve_kind;
