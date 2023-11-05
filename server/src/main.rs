mod commons;
mod server;
mod client;

use std::collections::HashMap;
use std::env::args;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::client::{Client, ClientMap};
use crate::commons::arguments::process_arguments;
use crate::server::Server;

fn main() -> Result<(), ()> {
    let argv = args().collect::<Vec<String>>();
    let mut config: Vec<String> = Vec::new();

    if process_arguments(argv, &mut config).is_err() {
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
            client_stream
                .set_read_timeout(Some(Duration::new(10, 0)))
                .expect("error");
            client_stream
                .set_write_timeout(Some(Duration::new(1, 0)))
                .expect("error");
            let _result = handle_client(&mut client_stream, &server);
        });
    }
}

fn handle_client(stream: &mut TcpStream, server: &Arc<Mutex<Server>>) -> std::io::Result<()> {
    let (tx_client_id, rx_client_id): (Sender<Option<String>>, Receiver<Option<String>>) =
        channel();

    println!("llegue :)");
    Ok(())
}
