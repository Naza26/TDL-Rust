use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use serde_json::Value;

// Reads constantly from buffer until connection to server is lost
pub fn listen_from(socket: &mut TcpStream) {
    let mut reader = BufReader::new(socket.try_clone().unwrap());
    std::thread::spawn(move || loop {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Err(e) => {
                println!("Error while reading from socket!: {}", e);
                break;
            }
            Ok(_m) => {
                // Deserialize the JSON string
                println!("{:?}", &buf); //todo: sacar
                let json = serde_json::from_str::<Value>(&buf).unwrap();
                if json["type"] == "CONNECTED" {
                    println!("Connected to server");
                } else if json["type"] == "ROOM_ADDED" {
                    let data = serde_json::from_value::<Value>(json["data"].clone()).unwrap();
                    let room_id: String = serde_json::from_value::<String>(data["room_id"].clone()).unwrap();
                    println!("Added to room id: {}", room_id);
                } else if json["type"] == "ROOM_STARTED" {
                    println!("Room started");
                } else if json["type"] == "MESSAGE" {
                    let msg: String = serde_json::from_str::<String>(&json["data"].to_string()).unwrap();
                    println!("Received chat message: {}", msg);
                }
                buf.clear();
            }
        };
    });
}