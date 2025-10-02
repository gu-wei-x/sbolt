use crate::types::error::CompileError;
use proc_macro2::TokenStream;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub(in crate::codegen::compiler) fn create_target_dir(base: &PathBuf, sub: &str) -> PathBuf {
    let target_dir = base.join(sub);
    if !target_dir.exists() {
        _ = std::fs::create_dir_all(&target_dir);
    }
    target_dir
}

pub(in crate::codegen::compiler) fn get_dir_name<P: AsRef<Path>>(path: &P) -> Option<String> {
    path.as_ref()
        .file_name()
        .and_then(|s| s.to_str().map(|s| s.to_string()))
}

pub(in crate::codegen::compiler) fn get_file_name<P: AsRef<Path>>(path: &P) -> Option<String> {
    path.as_ref()
        .file_stem()
        .and_then(|s| s.to_str().map(|s| s.to_string()))
}

pub(in crate::codegen::compiler) fn match_file_with_ext(path: &PathBuf, exts: &[String]) -> bool {
    path.extension()
        .map_or(false, |e| exts.contains(&e.to_string_lossy().into_owned()))
}

pub(in crate::codegen::compiler) fn write_code_to_file(
    file_path: &PathBuf,
    token_stream: &TokenStream,
) -> Result<(), CompileError> {
    let mut generated_file = File::create(&file_path)?;
    #[cfg(feature = "pretty")]
    {
        let syntax_tree = syn::parse_file(&token_stream.to_string())?;
        writeln!(generated_file, "{}", prettyplease::unparse(&syntax_tree))?;
    }
    #[cfg(not(feature = "pretty"))]
    {
        writeln!(generated_file, "{}", token_stream.to_string())?;
    }

    Ok(())
}
