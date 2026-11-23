use crate::codegen::{
    CompilerOptions,
    compiler::fsutil,
    parser::html::compact_html_whitespace,
    types::{Block, Template},
};
use crate::types::{
    error::CompileError, normalize_path_to_view_key, resolve_layout_to_view_keys, result,
    template::Kind,
};
use std::{collections::HashSet, fs, path::PathBuf};

pub(in crate::codegen::compiler) fn default_render_context_known(block: &Block<'_>) -> bool {
    let mut markup = String::new();
    let mut render_offset = None;
    if !collect_layout_markup(block, &mut markup, &mut render_offset) {
        return false;
    }
    let Some(offset) = render_offset else {
        return false;
    };
    compact_html_whitespace(&markup[..offset], false).1 && compact_html_whitespace(&markup, false).1
}

fn collect_layout_markup(
    block: &Block<'_>,
    markup: &mut String,
    render_offset: &mut Option<usize>,
) -> bool {
    match block {
        Block::KCONTENT(span) | Block::KINLINEDCONTENT(span) if span.is_simple() => {
            markup.push_str(&span.content());
            true
        }
        Block::KROOT(span) | Block::KCONTENT(span) => span
            .blocks()
            .iter()
            .all(|block| collect_layout_markup(block, markup, render_offset)),
        Block::KRENDER(span) if span.blocks().is_empty() => {
            render_offset.replace(markup.len()).is_none()
        }
        Block::KCOMMENT(_) | Block::KLAYOUT(_) | Block::KMODEL(_) | Block::KUSE(_) => true,
        _ => false,
    }
}

pub(in crate::codegen::compiler) fn layout_context_known(
    block: &Block<'_>,
    view_name: &str,
    options: &CompilerOptions,
) -> result::Result<bool> {
    let mut visited = HashSet::from([view_name.to_string()]);
    resolve_layout_context(block, view_name, options, &mut visited)
}

fn resolve_layout_context(
    block: &Block<'_>,
    view_name: &str,
    options: &CompilerOptions,
    visited: &mut HashSet<String>,
) -> result::Result<bool> {
    let Some(layout) = block
        .span()
        .blocks()
        .iter()
        .find(|block| matches!(block, Block::KLAYOUT(_)))
    else {
        return Ok(true);
    };
    for candidate in resolve_layout_to_view_keys(&layout.content(), view_name) {
        let Some(key) = normalize_path_to_view_key(&candidate) else {
            continue;
        };
        let Some((source, path)) = find_html_layout(&key, options)? else {
            continue;
        };
        if !visited.insert(key.clone()) {
            return Ok(false);
        }
        let template = Template::from(&source, None, Kind::KHTML, options)
            .map_err(|error| error.with_file(&path))?;
        if !default_render_context_known(template.block()) {
            return Ok(false);
        }
        return resolve_layout_context(template.block(), &key, options, visited);
    }
    Ok(false)
}

fn find_html_layout(
    key: &str,
    options: &CompilerOptions,
) -> result::Result<Option<(String, PathBuf)>> {
    let Some((logical_path, _)) = key.rsplit_once("::") else {
        return Ok(None);
    };
    let components: Vec<_> = logical_path.split("::").collect();
    if components.len() < 2
        || components
            .iter()
            .any(|part| part.is_empty() || part.contains(['.', '\\']))
    {
        return Ok(None);
    }
    for source_dir in options.source_dirs() {
        if fsutil::get_dir_name(source_dir).as_deref() != Some(components[0]) {
            continue;
        }
        if let Some(source) =
            find_html_source(&PathBuf::from(source_dir), &components[1..], options)?
        {
            return Ok(Some(source));
        }
    }
    Ok(None)
}

fn find_html_source(
    directory: &PathBuf,
    components: &[&str],
    options: &CompilerOptions,
) -> result::Result<Option<(String, PathBuf)>> {
    let entries =
        fs::read_dir(directory).map_err(|error| CompileError::from(error).with_file(directory))?;
    for entry in entries {
        let entry = entry.map_err(|error| CompileError::from(error).with_file(directory))?;
        let path = entry.path();
        if components.len() > 1 {
            if entry.file_name() != components[0] {
                continue;
            }
            if entry
                .metadata()
                .map_err(|error| CompileError::from(error).with_file(&path))?
                .is_dir()
            {
                return find_html_source(&path, &components[1..], options);
            }
            return Ok(None);
        }
        if fsutil::get_file_name(&path).as_deref() != Some(components[0])
            || fsutil::get_template_kind_from_ext(&path, options.extensions()) != Some(Kind::KHTML)
            || !entry
                .metadata()
                .map_err(|error| CompileError::from(error).with_file(&path))?
                .is_file()
        {
            continue;
        }
        let source = fs::read_to_string(&path)
            .map_err(|error| CompileError::from(error).with_file(&path))?;
        return Ok(Some((source, path)));
    }
    Ok(None)
}
