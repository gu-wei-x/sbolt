use indexmap::map;

#[derive(Debug)]
pub(in crate::codegen) struct Array {
    items: Vec<JObject>,
    is_closed: bool,
}

#[derive(Debug)]
pub(in crate::codegen) struct Map {
    items: map::IndexMap<String, JObject>,
    is_closed: bool,
}

#[derive(Debug)]
pub(in crate::codegen) enum JObject {
    KARRAY(Array),
    KMap(Map),
    KValue(String),
}

impl Default for JObject {
    fn default() -> Self {
        JObject::new_map()
    }
}

impl JObject {
    pub(in crate::codegen::parser::json) fn new_array() -> Self {
        JObject::KARRAY(Array {
            items: vec![],
            is_closed: false,
        })
    }

    pub(in crate::codegen::parser::json) fn new_map() -> Self {
        JObject::KMap(Map {
            items: map::IndexMap::new(),
            is_closed: false,
        })
    }

    pub(in crate::codegen::parser::json) fn new_value(value: &str) -> Self {
        JObject::KValue(value.into())
    }
}
