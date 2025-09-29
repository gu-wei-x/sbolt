mod block;
pub(crate) mod cgresult;
mod compiler;
mod options;
mod result;
mod template;

pub(crate) mod module;
pub(crate) mod registry;
#[cfg(test)]
pub(crate) mod tests;

pub use self::compiler::Compiler;
pub use self::options::CompilerOptions;
pub use self::result::CompileResult;
