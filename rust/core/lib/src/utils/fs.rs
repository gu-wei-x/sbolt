use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use proc_macro2::TokenStream;

pub(crate) fn generate_code_with_content(
    file_path: &PathBuf,
    token_stream: &TokenStream,
) -> Result<(), String> {
    let mut generated_file = File::create(&file_path).unwrap();
    #[cfg(feature = "pretty")]
    {
        let syntax_tree = syn::parse_file(&token_stream.to_string()).unwrap();
        _ = writeln!(generated_file, "{}", prettyplease::unparse(&syntax_tree));
    }

    #[cfg(not(feature = "pretty"))]
    {
        _ = writeln!(generated_file, "{}", token_stream.to_string());
    }

    Ok(())
}

pub fn normalize_name(name: &str, prefix: &str, postfix: &str) -> String {
    let normalized_name = name.replace('\\', "_").replace('/', "_");
    let mut name = prefix.to_string();
    name.push_str(&normalized_name);
    name.push_str(postfix);
    name
}

pub fn create_target_dir(base: &PathBuf, sub: &str) -> PathBuf {
    let target_dir = base.join(sub);
    if !target_dir.exists() {
        _ = fs::create_dir_all(&target_dir);
    }
    target_dir
}

pub(crate) fn match_file_with_ext(path: &PathBuf, exts: &[String]) -> bool {
    path.extension()
        .map_or(false, |e| exts.contains(&e.to_string_lossy().into_owned()))
}

pub(crate) fn get_dir_name<P: AsRef<Path>>(path: &P) -> Option<String> {
    path.as_ref()
        .file_name()
        .and_then(|s| s.to_str().map(|s| s.to_string()))
}

pub(crate) fn get_file_name<P: AsRef<Path>>(path: &P) -> Option<String> {
    path.as_ref()
        .file_stem()
        .and_then(|s| s.to_str().map(|s| s.to_string()))
}
