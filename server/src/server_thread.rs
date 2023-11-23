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
    AddClientToRoom(u8),
    ///
    /// Send message from a client
    /// u8: client id
    /// String: msg
    ///
    SendMessageFromClient(u8, String),
    ///
    /// Finish chatting with the client
    /// u8: client id
    ///
    FinishChattingFromClient(u8),
    ///
    /// Choose participants
    /// u8: client id
    /// Vec<u8>: list of participants the client chose
    ///
    ChooseParticipants(u8, Vec<u8>)
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
                },
                ServerMessage::AddClientToRoom(id) => {
                    server.insert_client_to_room(id).unwrap();
                },
                ServerMessage::SendMessageFromClient(client_id, msg) => {
                    server.send_message_from_client(client_id, msg);
                },
                ServerMessage::FinishChattingFromClient(client_id) => {
                    server.finish_chat_room(client_id);
                },
                ServerMessage::ChooseParticipants(client_id, participants) => {
                    server.choose_participant_from_client(client_id, participants);
                }
            }
        }
    })
}