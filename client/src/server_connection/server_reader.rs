use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Duration;
use serde_json::Value;
use crate::commons::client_state::ClientState;

// Reads constantly from buffer until connection to server is lost
pub fn listen_from(socket: TcpStream, client_state: Arc<Mutex<ClientState>>, rx: Receiver<bool>) {
    socket.set_read_timeout(Some(Duration::new(2, 0))).unwrap();
    let mut reader = BufReader::new(socket.try_clone().unwrap());
    loop {
        let message: Result<bool, RecvTimeoutError> = rx.recv_timeout(Duration::from_millis(500));
        if let Ok(should_finish_app) = message {
            if should_finish_app {
                return;
            }
        }

        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(_m) => {
                let json = serde_json::from_str::<Value>(&buf).unwrap();
                if json["type"] == "CONNECTED" {
                    println!("Connected to server");
                } else if json["type"] == "ROOM_ADDED" {
                    let data = serde_json::from_value::<Value>(json["data"].clone()).unwrap();
                    let room_id: String = serde_json::from_value::<String>(data["room_id"].clone()).unwrap();
                    println!("Added to room id: {}", room_id);
                } else if json["type"] == "ROOM_STARTED" {
                    println!("Room started");
                }
                else if json["type"] == "CHAT_ROOM_STARTED" {
                    let client_id: u8 = serde_json::from_str::<u8>(&json["data"].to_string()).unwrap();
                    println!("Started chatting with client id: {}", client_id);
                    println!("Type 'quit' to finish chat");
                    if let Ok(mut client_state_locked) = client_state.lock() {
                        *client_state_locked = ClientState::Chatting;
                    }
                }
                else if json["type"] == "MESSAGE" {
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
                } else if json["type"] == "MATCHING_RESULT" {
                    let clients_matched: Vec<u8> = serde_json::from_str::<Vec<u8>>(&json["data"].to_string()).unwrap();
                    println!("Clients matched: {:?}", clients_matched);
                    if let Ok(mut client_state_locked) = client_state.lock() {
                        *client_state_locked = ClientState::AddNewRoom;
                    }
                    println!("Do you want to enter another room? [yes/no]");
                }
                buf.clear();
            }
            Err(_e) => {}
        };
    }
}