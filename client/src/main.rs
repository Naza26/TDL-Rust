mod commons;

use std::collections::HashMap;
use std::env::args;
use std::io::{self, BufRead, BufReader, ErrorKind, Stdin, stdin, Write};
use std::net::TcpStream;
use crate::commons::process_client_info::process_client_info;

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
    let stdin: Stdin = io::stdin();
    let mut socket = TcpStream::connect(address).unwrap();
    let client_info = process_client_info();
    write_to_socket(&create_client_info_string(client_info.unwrap()), &mut socket);
    println!("Esperando");
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        client_run(l, &mut socket);
    }
}

fn client_run(line: String, socket: &mut TcpStream) -> std::io::Result<()> {
    write_to_socket(&line, socket);
    let datos = {"nombre:, pais:, edad"};

    listen_from(socket);

    Ok(())
}

fn write_to_socket(msg: &str, socket: &mut TcpStream)
{
    let payload = msg.as_bytes().to_vec();
    socket.write_all(payload.as_slice());
    socket.write_all(b"\n");
    socket.flush();
    println!("written");
}

fn create_client_info_string(client_info: HashMap<String, String>) -> String {
    let mut msg = "{".to_string();
    for (key, value) in client_info {
        msg += &key;
        msg += ":";
        msg += &value;
        msg += ",";
    }
    msg += "}";
    return msg;
}

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
            Ok(m) => {
                // if m > 0 i have something to read from the socket
                if m > 0 {
                    buf.pop();
                    buf.pop();
                    println!("Incomming message from server: {:?}", buf);
                } else {
                    // the server was closed, therefore the socket is broken
                    continue;
                }
            }
        };
    });
}

fn main() {
    connect();
}
