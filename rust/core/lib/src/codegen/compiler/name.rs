pub(in crate::codegen::compiler) fn create_name_space(
    ns_prefix: &Option<String>,
    name: &str,
) -> String {
    match ns_prefix {
        None => name.to_string(),
        Some(prefix) => {
            if prefix.is_empty() {
                name.to_string()
            } else {
                let mut ns = prefix.to_string();
                ns.push_str(&format!("::{}", name));
                ns
            }
        }
    }
}

pub(in crate::codegen::compiler) fn create_type_full_name(
    type_name: &str,
    mod_name: &str,
) -> String {
    format!("crate::{}::{}", mod_name, type_name)
}

pub(in crate::codegen::compiler) fn create_view_type_full_name(
    view_key: &str,
    view_name: &str,
    mod_name: &str,
) -> String {
    let namespace = view_key
        .rsplit_once("::")
        .map(|(prefix, _)| prefix.to_string());
    create_type_full_name(&create_name_space(&namespace, view_name), mod_name)
}

pub(in crate::codegen::compiler) fn create_view_type_name(name: &str) -> String {
    let prefix = name
        .split("::")
        .flat_map(|part| part.split('_'))
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>();
    format!("{}View", prefix)
}
