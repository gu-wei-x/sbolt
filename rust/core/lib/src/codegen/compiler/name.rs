pub(crate) fn normalize_to_type_name(name: &str) -> String {
    let prefix = name
        .split("::")
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

pub(crate) fn create_full_name(prefix: &Option<String>, name: &str) -> String {
    match prefix {
        None => return name.to_string(),
        Some(p) => {
            if p.is_empty() {
                name.to_string()
            } else {
                let mut full_name = p.to_string();
                full_name.push_str(&format!("::{}", name));
                return full_name;
            }
        }
    }
}

pub(crate) fn create_mode_prefix(path: &str) -> String {
    path.replace("_", "::")
}

mod test {
    #[test]
    fn normalize_to_type_name() {
        assert_eq!(super::normalize_to_type_name("index"), "IndexView");
    }
}
