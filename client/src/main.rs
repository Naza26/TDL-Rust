mod commons;
mod server_connection;

use std::env::args;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::commons::client_state::ClientState;
use crate::commons::input::start_reading_input;

use crate::server_connection::server_reader::listen_from;
use crate::server_connection::server_writer::send_client_info;

static CLIENT_ARGS: usize = 3;

pub fn connect() {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != CLIENT_ARGS {
        println!("Invalid amount of arguments!");
        let app_name = &argv[0];
        println!("{:?} <host> <port>", app_name);
        return;
    }
    let address = argv[1].clone() + ":" + &argv[2];
    println!("Connecting to {:?}", address);

    let mut socket = TcpStream::connect(address).unwrap();

    let client_state = Arc::new(Mutex::new(ClientState::Waiting));

    send_client_info(&mut socket);

    let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();
    start_reading_input(socket.try_clone().unwrap(), client_state.clone(), tx);
    listen_from(socket.try_clone().unwrap(), client_state, rx);
}

fn main() {
    connect();
}


/*

{
    type: "CONNECTED"
}
{
    type: "ROOM_ADDED",
    data: {
        room_id: 2
    }
}
{
    type: "ROOM_STARTED"
}
{
    type: "MESSAGE",
    data: "blablabla"
}
 */
