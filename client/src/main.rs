mod commons;
mod server_reader;
mod server_writer;

use std::env::args;
use std::io::{BufRead, Stdin, stdin};
use std::net::TcpStream;

use crate::server_reader::listen_from;
use crate::server_writer::{client_run, send_client_info};

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

    listen_from(&mut socket);

    send_client_info(&mut socket);

    let stdin: Stdin = stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        client_run(l, &mut socket).expect("Panic");
    }
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
