#![cfg(test)]
use crate::types::DataStore;

// cargo test test_primitive_type -- --nocapture
#[test]
fn primitive_type() {
    let store = DataStore::<String>::new();
    store.set("k1", "v1".to_string());
    store.set("k2", 1);

    assert!(!store.set("k1", "v2".to_string()));
    assert!(!store.set("k2", "v2".to_string()));
    assert!(!store.set("k2", 2));

    assert_eq!(store.get::<String>("k1"), Some(&"v1".to_string()));
    assert_eq!(store.get::<i32>("k2"), Some(&1));
}

#[test]
fn custom_type() {
    #[derive(PartialEq, Debug)]
    struct CustomType {
        value: String,
    }

    let store = DataStore::<String>::new();
    store.set(
        "k1",
        CustomType {
            value: "v1".to_string(),
        },
    );

    assert_eq!(
        store.get::<CustomType>("k1"),
        Some(&CustomType {
            value: "v1".to_string()
        })
    );
}
