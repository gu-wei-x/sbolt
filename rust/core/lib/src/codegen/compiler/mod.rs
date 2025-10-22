mod compiler;
mod context;
mod error;
mod fsutil;
mod module;
mod name;
mod optimizer;
mod options;
mod registry;
mod result;
mod types;

#[cfg(test)]
mod tests;

// re-export
pub use self::compiler::Compiler;
pub use self::options::CompilerOptions;
pub use self::result::CompileResult;
