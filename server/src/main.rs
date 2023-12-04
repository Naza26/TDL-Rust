mod commons;
mod server;
mod client;
mod client_thread;
mod server_thread;

use std::env::args;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use crate::client_thread::handle_client;
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

    println!("Server listening on port {}", &config[0]);
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
