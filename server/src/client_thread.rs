use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::sync::mpsc::{Sender};
use crate::client::ClientInfo;
use crate::server_thread::ServerMessage;
use serde_json::Value;

pub fn handle_client(stream: &mut TcpStream, server_sender: Sender<ServerMessage>, client_id: u8) -> std::io::Result<()> {

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buf = String::new();
    loop {
        match reader.read_line(&mut buf) {
            Ok(_m) => {
                // Deserialize the JSON string
                //println!("{:?}", &buf); // todo: sacar
                let json_result = serde_json::from_str::<Value>(&buf);
                if json_result.is_err() {
                    return Ok(())
                }
                let json = json_result.unwrap();
                if json["type"] == "CONNECT" {
                    let data = serde_json::from_value::<Value>(json["data"].clone()).unwrap();
                    let age: String = serde_json::from_value::<String>(data["age"].clone()).unwrap();
                    let name: String = serde_json::from_value::<String>(data["name"].clone()).unwrap();
                    let country: String = serde_json::from_value::<String>(data["country"].clone()).unwrap();

                    let client_info: ClientInfo = ClientInfo::new(name, age, country);
                    let client_stream = stream.try_clone().unwrap();

                    server_sender.send(ServerMessage::AddClient(client_id, client_info, client_stream)).unwrap();
                    server_sender.send(ServerMessage::AddClientToRoom(client_id)).unwrap();

                } else if json["type"] == "MESSAGE" {
                    let msg: String = serde_json::from_str::<String>(&json["data"].to_string()).unwrap();
                    server_sender.send(ServerMessage::SendMessageFromClient(client_id, msg)).unwrap();
                } else if json["type"] == "QUIT_CHATTING" {
                    server_sender.send(ServerMessage::FinishChattingFromClient(client_id)).unwrap();
                } else if json["type"] == "CHOOSE_PARTICIPANTS" {
                    let data = serde_json::from_value::<Vec<u8>>(json["data"].clone()).unwrap();
                    server_sender.send(ServerMessage::ChooseParticipants(client_id, data)).unwrap();
                }
                else if json["type"] == "ADD_ROOM" {
                    server_sender.send(ServerMessage::AddClientToRoom(client_id)).unwrap();
                }
                buf.clear();
            },
            Err(e) => println!("{}", e)
        }
    }
}
