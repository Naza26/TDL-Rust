use std::io::Write;
use std::net::TcpStream;
use crate::commons::messages::{create_client_info_string, create_client_message};
use crate::commons::process_client_info::process_client_info;

pub fn client_run(line: String, socket: &mut TcpStream) -> std::io::Result<()> {

    let payload = create_client_message(line);
    write_to_socket(&payload, socket);

    Ok(())
}

pub fn write_to_socket(msg: &str, socket: &mut TcpStream)
{
    let payload = msg.as_bytes().to_vec();
    socket.write_all(payload.as_slice()).unwrap();
    socket.write_all(b"\n").unwrap();
    socket.flush().unwrap();
}

pub fn send_client_info(socket: &mut TcpStream) {
    let client_info = process_client_info();
    write_to_socket(&create_client_info_string(client_info.unwrap()), socket);
}