extern crate serde_json;

use serde_json::Value as JsonValue;

fn main() {
    let json_str = r#"
        {
            "name":"michael",
            "age": 24,
            "is_male": true
        }
    "#;

    let result = serde_json::from_str(json_str);

    if result.is_ok() {
        let parsed: serde_json::Value = result.unwrap();
        println!("Name: {}", parsed["name"].as_str().unwrap());
    } else {
        println!("Failed to parse JSON.");
    }
}
