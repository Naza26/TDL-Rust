mod commons;
mod server;
mod client;

use std::collections::HashMap;
use std::env::args;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::client::{ClientMap,ClientInfo};
use crate::commons::arguments::arguments_cant_be_processed;
use crate::server::Server;

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
    let clients: ClientMap = Arc::new(Mutex::new(HashMap::new()));
    let server = Server::new(address, &clients);

    let server = Arc::new(Mutex::new(server.unwrap()));

    let listener = TcpListener::bind(address)?;

    loop {
        let connection = listener.accept()?;

        let server = Arc::clone(&server);

        thread::spawn(move || {
            let mut client_stream: TcpStream = connection.0;
            let _result = handle_client(&mut client_stream, &server);
        });
    }
}

fn handle_client(stream: &mut TcpStream, _server: &Arc<Mutex<Server>>) -> std::io::Result<()> {
    let (_tx_client_id, _rx_client_id): (Sender<Option<String>>, Receiver<Option<String>>) =
        channel();

    println!("llegue :)");

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buf = String::new();
    match reader.read_line(&mut buf) {
        Ok(_m) => {
            // Deserialize the JSON string
            match serde_json::from_str::<ClientInfo>(&buf) {
                Ok(client_info) => {
                    println!("{:?}", client_info);
                    println!("Name: {}", client_info.name.unwrap_or_default());
                    println!("Country: {}", client_info.country.unwrap_or_default());
                    println!("Age: {}", client_info.age.unwrap_or_default());
                }
                Err(e) => {
                    eprintln!("Error deserializing: {}", e);
                }
            }
        },
        Err(e) => println!("{}", e)
    }
    Ok(())
}

