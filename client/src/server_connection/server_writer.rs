use std::io::Write;
use std::net::TcpStream;
use crate::commons::messages::{create_add_new_room_message, create_choose_participants_message, create_client_info_string, create_client_message, create_quit_chatting_string};
use crate::commons::process_client_info::process_client_info;


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

pub fn send_message(line: String, socket: &mut TcpStream) {
    let payload = create_client_message(line);
    write_to_socket(&payload, socket);
}

pub fn send_quit_message(socket: &mut TcpStream) {
    write_to_socket(&create_quit_chatting_string(), socket);
}

pub fn send_choose_participants_message(line: String, socket: &mut TcpStream) {
    write_to_socket(&create_choose_participants_message(line), socket);
}

pub fn send_add_room_message(socket: &mut TcpStream) {
    write_to_socket(&create_add_new_room_message(), socket);
}
