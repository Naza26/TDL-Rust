use std::collections::HashMap;
use serde_json::Value;

pub fn create_connected_message() -> Vec<u8> {
    let mut msg = HashMap::new();
    msg.insert("type".to_string(), "CONNECTED".to_string());

    let mut message = serde_json::to_value(&msg).expect("Failed to serialize connected message to JSON").to_string();
    message += "\n";
    message.as_bytes().to_vec()
}

pub fn create_room_added_message(room_id: u8) -> Vec<u8> {
    let mut msg = HashMap::new();
    msg.insert("type".to_string(), Value::String("ROOM_ADDED".to_string()));

    let mut data_hashmap = HashMap::new();
    data_hashmap.insert("room_id".to_string(), room_id.to_string());
    let data: Value = serde_json::to_value(&data_hashmap).expect("Failed to serialize ClientInfo to JSON");
    msg.insert("data".to_string(), data);

    let mut message = serde_json::to_value(&msg).expect("Failed to serialize room added message to JSON").to_string();
    message += "\n";
    message.as_bytes().to_vec()
}

pub fn create_room_started_message() -> Vec<u8> {
    let mut msg = HashMap::new();
    msg.insert("type".to_string(), "ROOM_STARTED".to_string());

    let mut message = serde_json::to_value(&msg).expect("Failed to serialize connected message to JSON").to_string();
    message += "\n";
    message.as_bytes().to_vec()
}

pub fn create_client_message(line: String) -> Vec<u8> {
    let mut msg = HashMap::new();
    msg.insert("type".to_string(), "MESSAGE".to_string());
    msg.insert("data".to_string(), line);

    let mut message = serde_json::to_value(&msg).expect("Failed to serialize JSON").to_string();
    message += "\n";
    message.as_bytes().to_vec()
}


