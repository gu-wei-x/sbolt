/// Normalize a file path to a Rust struct name.
/// "comp/index" => "CompIndexView"
pub(crate) fn normalize_path_to_struct_name(name: &str, suffix: &str) -> String {
    let name = name.trim_matches(|c| c == '/' || c == '\\' || c == '_');
    let prefix = name
        .split(&['/', '\\', '_'])
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>();
    format!("{}{}", prefix, suffix)
}

pub(crate) fn normalize_path_to_view_name(name: &str) -> String {
    normalize_path_to_struct_name(name, "View")
}

mod test {
    #[test]
    fn test_normalize_path_to_struct_name() {
        assert_eq!(
            super::normalize_path_to_struct_name("comp/index", ""),
            "CompIndex"
        );
        assert_eq!(
            super::normalize_path_to_struct_name("comp_index", ""),
            "CompIndex"
        );
        assert_eq!(
            super::normalize_path_to_struct_name("comp/index", "View"),
            "CompIndexView"
        );
        assert_eq!(
            super::normalize_path_to_struct_name("comp_index", "View"),
            "CompIndexView"
        );
    }
}
