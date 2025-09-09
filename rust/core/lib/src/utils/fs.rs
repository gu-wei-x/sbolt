use std::ffi::OsString;
use std::fs::{self, File, metadata, read_link};
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

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

pub(crate) fn get_files_with_extension<P: AsRef<Path>>(
    path: P,
    exts: &[OsString],
    files: &mut Vec<PathBuf>,
) {
    if let Ok(md) = metadata(&path) {
        // not a dir.
        if !md.is_dir() {
            return;
        }
    } else {
        // doesn't exist.
        return;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    get_files_with_extension(&path, exts, files);
                } else if meta.is_file() {
                    if let Some((_name, path)) = match_file_with_ext(&path, exts) {
                        files.push(path);
                    }
                } else if meta.file_type().is_symlink() {
                    if let Ok(target) = read_link(&path) {
                        get_files_with_extension(target, exts, files);
                    }
                }
            }
        }
    }
}

pub(crate) fn path_to_name(
    base_path: &PathBuf,
    path: &PathBuf,
    prefix: &str,
    postfix: &str,
) -> String {
    let file_name = path
        .with_extension("")
        .strip_prefix(base_path)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "_")
        .replace('/', "_");

    let mut name = prefix.to_string();
    name.push_str(&file_name);
    name.push_str(postfix);
    name
}

pub fn normalize_name(name: &str, prefix: &str, postfix: &str) -> String {
    let normalized_name = name.replace('\\', "_").replace('/', "_");

    let mut name = prefix.to_string();
    name.push_str(&normalized_name);
    name.push_str(postfix);
    name
}

fn match_file_with_ext(path: &PathBuf, exts: &[OsString]) -> Option<(OsString, PathBuf)> {
    if path.is_file() {
        if path
            .extension()
            .map_or(false, |e| exts.contains(&e.to_os_string()))
        {
            match path.file_stem() {
                Some(file_stem) => Some((file_stem.into(), path.to_path_buf())),
                None => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}
