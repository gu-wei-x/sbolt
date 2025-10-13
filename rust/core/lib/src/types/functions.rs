pub fn normalize_path_to_view_key(path: &str) -> Option<String> {
    if path.is_empty() {
        return None;
    }

    let normalized_name = path.trim_matches('/').replace("/", "::");
    let mut parts = normalized_name.split("::").peekable();
    let mut result = String::new();
    while let Some(part) = parts.next() {
        result.push_str(part);
        result.push_str("::");
        match parts.peek() {
            None => {
                // append name for last one.
                let mut chars = part.chars();
                if let Some(first_char) = chars.next() {
                    result.push_str(&first_char.to_uppercase().collect::<String>());
                    result.push_str(chars.as_str());
                    result.push_str("View");
                }
                break;
            }
            _ => {
                continue;
            }
        }
    }
    Some(result)
}

pub fn resolve_layout_to_view_keys(layout_path: &str, view_name: &str) -> Vec<String> {
    // 1. /exp: absolute path, remove / and return
    let mut result = vec![];
    if layout_path.starts_with('/') {
        result.push(layout_path[1..layout_path.len()].to_string());
        return result;
    }

    // view folder, the last 2 is the view mod and name, need to ignore.
    let mut parts: Vec<String> = view_name.split("::").map(|s| s.to_string()).collect();
    parts.pop();
    parts.pop();

    // 2. ~/exp: relative to current dir
    // => current/exp
    if layout_path.starts_with("~/") {
        let layout_path = &layout_path[2..layout_path.len()];
        parts.push(layout_path.to_string());
        let path = parts.join("/");
        result.push(path);
        return result;
    }

    // 3. exp1/exp2: need to fallback to each parent level
    // path/layout, "base::test1::test2::Test2View"=> test1/[path/layout], base/[path/layout], [path/layout]
    let mut index = parts.len();
    while index > 0 {
        let path = parts[0..index].join("/");
        result.push(format!("{path}/{layout_path}"));
        index -= 1;
    }

    result.push(layout_path.to_string());
    result
}
