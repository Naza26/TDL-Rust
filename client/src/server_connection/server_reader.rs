use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use serde_json::Value;
use crate::commons::client_state::ClientState;

// Reads constantly from buffer until connection to server is lost
pub fn listen_from(socket: TcpStream, client_state: Arc<Mutex<ClientState>>) {
    let mut reader = BufReader::new(socket.try_clone().unwrap());
    loop {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Err(e) => {
                println!("Error while reading from socket!: {}", e);
                break;
            }
            Ok(_m) => {
                // Deserialize the JSON string
                //println!("{:?}", &buf); //todo: sacar
                let json = serde_json::from_str::<Value>(&buf).unwrap();
                if json["type"] == "CONNECTED" {
                    println!("Connected to server");
                } else if json["type"] == "ROOM_ADDED" {
                    let data = serde_json::from_value::<Value>(json["data"].clone()).unwrap();
                    let room_id: String = serde_json::from_value::<String>(data["room_id"].clone()).unwrap();
                    println!("Added to room id: {}", room_id);
                } else if json["type"] == "ROOM_STARTED" {
                    println!("Room started");
                    println!("Type 'quit' to finish chat"); //todo: mover a sala de chat
                    if let Ok(mut client_state_locked) = client_state.lock() {
                        *client_state_locked = ClientState::Chatting;
                    }
                } else if json["type"] == "MESSAGE" {
                    let msg: String = serde_json::from_str::<String>(&json["data"].to_string()).unwrap();
                    println!("Received chat message: {}", msg);
                } else if json["type"] == "QUIT_CHATTING" {
                    println!("Chat finished");
                    if let Ok(mut client_state_locked) = client_state.lock() {
                        *client_state_locked = ClientState::Waiting;
                    }
                } else if json["type"] == "PARTICIPANTS" {
                    let participants: HashMap<u8, String> = serde_json::from_str::<HashMap<u8, String>>(&json["data"].to_string()).unwrap();
                    println!("Please type the list of ids of people you wish to match");
                    println!("{:?}", participants);
                    if let Ok(mut client_state_locked) = client_state.lock() {
                        *client_state_locked = ClientState::ChoosingMatch;
                    }
                } else if json["type"] == "PARTICIPANTS_CHOSEN" {
                    println!("List of participants sent to server, waiting for match");
                    if let Ok(mut client_state_locked) = client_state.lock() {
                        *client_state_locked = ClientState::Waiting;
                    }
                }
                buf.clear();
            }
        };
    }
}