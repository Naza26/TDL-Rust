//! Run client from terminal. Used for testing
use std::env::args;
use std::io::{self, BufRead, BufReader, ErrorKind, Stdin};
use std::net::TcpStream;

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
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        // Check if the socket has been closed
        match socket.peer_addr() {
            Ok(_) => {
                // Socket is still open
                client_run(l, &mut socket);
            }
            Err(error) if error.kind() == ErrorKind::NotConnected => {
                println!("Socket has been closed, stop reading from stdin. {}", error);
                break;
            }
            Err(error) => {
                println!("Error ocurred with socket: {}", error)
            }
        }
    }
}

fn client_run(line: String, socket: &mut TcpStream) -> std::io::Result<()> {
    println!("{}", line);

    listen_from(socket);

    Ok(())
}

fn write_to_socket(msg: &str, socket: &mut TcpStream)
{
    println!("asdaa");
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
