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

pub fn create_chat_room_started_message(client_id: u8) -> Vec<u8> {
    let mut msg = HashMap::new();
    msg.insert("type".to_string(), Value::String("CHAT_ROOM_STARTED".to_string()));
    msg.insert("data".to_string(), Value::Number(client_id.into()));

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

pub fn create_list_participants_message(participants: HashMap<u8, String>) -> Vec<u8> {
    let mut msg = HashMap::new();
    msg.insert("type".to_string(), Value::String("PARTICIPANTS".to_string()));

    let data: Value = serde_json::to_value(participants).expect("Failed to serialize participants to JSON");
    msg.insert("data".to_string(), data);

    let mut message = serde_json::to_value(&msg).expect("Failed to serialize JSON").to_string();
    message += "\n";
    message.as_bytes().to_vec()
}

pub fn create_quit_chatting_message() -> Vec<u8> {
    let mut msg = HashMap::new();
    msg.insert("type".to_string(), Value::String("QUIT_CHATTING".to_string()));

    let mut message = serde_json::to_value(&msg).expect("Failed to serialize JSON").to_string();
    message += "\n";
    message.as_bytes().to_vec()
}

pub fn create_wait_new_chat_message() -> Vec<u8> {
    let mut msg = HashMap::new();
    msg.insert("type".to_string(), Value::String("WAIT_NEW_CHAT".to_string()));

    let mut message = serde_json::to_value(&msg).expect("Failed to serialize JSON").to_string();
    message += "\n";
    message.as_bytes().to_vec()
}

pub fn create_participants_chosen_message() -> Vec<u8> {
    let mut msg = HashMap::new();
    msg.insert("type".to_string(), Value::String("PARTICIPANTS_CHOSEN".to_string()));

    let mut message = serde_json::to_value(&msg).expect("Failed to serialize JSON").to_string();
    message += "\n";
    message.as_bytes().to_vec()
}

pub fn create_matching_result_message(matches: Vec<u8>) -> Vec<u8> {
    let mut msg = HashMap::new();
    msg.insert("type".to_string(), Value::String("MATCHING_RESULT".to_string()));
    msg.insert("data".to_string(), Value::Array(create_clients_match_vector(matches)));

    let mut message = serde_json::to_value(&msg).expect("Failed to serialize JSON").to_string();
    message += "\n";
    message.as_bytes().to_vec()
}

fn create_clients_match_vector(matches: Vec<u8>) -> Vec<Value> {
    let mut clients_match_vector = Vec::new();
    for m in matches {
        clients_match_vector.push(Value::Number(m.into()));
    }
    clients_match_vector
}
