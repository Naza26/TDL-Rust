mod commons;
mod server;
mod client;
mod client_thread;
mod server_thread;

use serde_json::Value;
use std::env::args;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use crate::client::ClientInfo;
use crate::commons::arguments::arguments_cant_be_processed;
use crate::server::Server;
use crate::server_thread::ServerMessage;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    let mut config: Vec<String> = Vec::new();

    if arguments_cant_be_processed(argv, &mut config) {
        return Err(());
    }

    let address = "0.0.0.0:".to_owned() + &config[0];

    server_run(&address).unwrap();
    Ok(())
}

fn server_run(address: &str) -> std::io::Result<()> {
    let server = Server::new().unwrap();

    let (server_sender, server_recv): (Sender<ServerMessage>, Receiver<ServerMessage>) =
        channel();
    server_thread::spawn_server_worker(server_recv, server);

    let listener = TcpListener::bind(address)?;
    let mut client_id: u8 = 1;

    loop {
        let connection = listener.accept()?;

        let server_sender = server_sender.clone();

        thread::spawn(move || {
            let mut client_stream: TcpStream = connection.0;
            let _result = handle_client(&mut client_stream, server_sender, client_id);
        });

        client_id += 1;
    }
}

fn handle_client(stream: &mut TcpStream, server_sender: Sender<ServerMessage>, client_id: u8) -> std::io::Result<()> {
    let (_tx_client_id, _rx_client_id): (Sender<Option<String>>, Receiver<Option<String>>) =
        channel();

    println!("llegue :)");

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buf = String::new();
    loop {
        match reader.read_line(&mut buf) {
            Ok(_m) => {
                // Deserialize the JSON string
                println!("{:?}", &buf);
                let json = serde_json::from_str::<Value>(&buf).unwrap();
                if json["type"] == "CONNECT" {
                    let age: String = json["data"]["age"].to_string();
                    let name: String = json["data"]["name"].to_string();
                    let country: String = json["data"]["country"].to_string();

                    let client_info: ClientInfo = ClientInfo::new(name, age, country);
                    println!("{:?}", client_info);
                    let client_stream = stream.try_clone().unwrap();

                    server_sender.send(ServerMessage::AddClient(client_id, client_info, client_stream)).unwrap();
                    server_sender.send(ServerMessage::AddClientToRoom(client_id)).unwrap();

                } else if json["type"] == "MESSAGE" {
                    let msg: String = serde_json::from_str::<String>(&json["data"].to_string()).unwrap();
                    println!("{:?}", msg);

                    server_sender.send(ServerMessage::SendMessageFromClient(client_id, msg)).unwrap();
                }
                buf.clear();
            },
            Err(e) => println!("{}", e)
        }
    }
}

/*

{
    type: "CONNECT",
    data: {
        age: 24,
        name: Naza,
        coutry: Argentina
    }
}
{
    type: "MESSAGE",
    data: "blablabla"
}
{
    type: "FINALIZE_CHAT"
}
 */

