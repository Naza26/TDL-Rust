use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use crate::client::ClientInfo;
use crate::server::Server;

#[derive(Debug)]
pub enum ServerMessage {
    ///
    /// Add client to server
    /// u8: client_id
    /// ClientInfo: client info
    /// TcpStream: client stream
    ///
    AddClient(u8,ClientInfo,TcpStream),
    ///
    /// Add client to an available room
    /// u8: id
    ///
    AddClientToRoom(u8)
}


pub fn spawn_server_worker(
    recv: Receiver<ServerMessage>,
    mut server: Server
) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let message: ServerMessage = recv.recv().unwrap();
            match message {
                ServerMessage::AddClient(client_id, client_info, stream) => {
                    server.add_client(client_id, client_info, stream);
                    println!("server worker received add client message with name");
                },
                ServerMessage::AddClientToRoom(id) => {
                    server.insert_client_to_room(id).unwrap();
                }
            }
        }
    })
}