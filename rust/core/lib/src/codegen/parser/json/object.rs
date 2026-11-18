#[derive(Debug)]
pub(in crate::codegen) enum JObject {
    KARRAY(Vec<JObject>),
    KMap(Vec<(String, JObject)>),
    KValue(String),
}

impl JObject {
    pub(in crate::codegen) fn to_compact_string(&self) -> String {
        let mut output = String::new();
        self.write_compact(&mut output);
        output
    }

    fn write_compact(&self, output: &mut String) {
        match self {
            Self::KARRAY(items) => {
                output.push('[');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    item.write_compact(output);
                }
                output.push(']');
            }
            Self::KMap(items) => {
                output.push('{');
                for (index, (name, value)) in items.iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    // Names retain their original quotes and escapes, including duplicate keys.
                    output.push_str(name);
                    output.push(':');
                    value.write_compact(output);
                }
                output.push('}');
            }
            Self::KValue(value) => output.push_str(value),
        }
    }
}
