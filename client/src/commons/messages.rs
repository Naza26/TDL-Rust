use std::collections::HashMap;
use serde_json::Value;

pub fn create_client_message(line: String) -> String {
    let mut msg = HashMap::new();
    msg.insert("type".to_string(), "MESSAGE".to_string());
    msg.insert("data".to_string(), line);

    serde_json::to_value(&msg).expect("Failed to serialize JSON").to_string()
}

pub fn create_client_info_string(client_info: HashMap<String, String>) -> String {
    let data: Value = serde_json::to_value(&client_info).expect("Failed to serialize ClientInfo to JSON");

    let mut msg = HashMap::new();
    msg.insert("type".to_string(), Value::String("CONNECT".to_string()));
    msg.insert("data".to_string(), data);

    serde_json::to_value(&msg).expect("Failed to serialize ClientInfo to JSON").to_string()
}
