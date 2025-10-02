mod block;
mod cgresult;
mod compiler;
mod error;
mod fsutil;
mod module;
mod name;
mod options;
mod registry;
mod result;
mod template;

#[cfg(test)]
mod tests;

// re-export
pub use self::compiler::Compiler;
pub use self::options::CompilerOptions;
pub use self::result::CompileResult;
