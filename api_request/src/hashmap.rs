use std::collections::HashMap;

pub fn hashmap_value() {
    let mut map = HashMap::new();

    // Insert key-value pairs
    map.insert("name", "Alice");
    map.insert("age", "30");

    // Access values
    if let Some(name) = map.get("name") {
        println!("Name: {}", name);
    }

    // Iterate over key-value pairs
    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }
}
