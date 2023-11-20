pub mod room;

use std::net::TcpStream;
use std::{
    thread::JoinHandle,
};
use std::io::Write;
use crate::client::{ClientInfo, Clients};
use crate::commons::messages;
use crate::server::room::Rooms;



pub struct Server {
    pub receiver_thread: Option<JoinHandle<()>>,
    pub registered_clients: Clients,
    pub rooms: Rooms
}

/// Implement Drop trait for server. This way, when the server
/// is dropped, the server thread will be closed
impl Drop for Server {
    fn drop(&mut self) {
        println!("Droping server ...");
        if let Some(thread) = self.receiver_thread.take() {
            if thread.join().is_err() {
                println!("Error while closing receiver thread from ");
            }
        }
    }
}

impl Server {
    pub fn new() -> Result<Server, ()> {
        Ok(Server {
            receiver_thread: None,
            registered_clients: Clients::new(),
            rooms: Rooms::new()
        })
    }

    pub fn add_client(&mut self, client_id: u8, client_info: ClientInfo, mut stream: TcpStream) {
        self.registered_clients.add_client(client_id, client_info, stream.try_clone().unwrap());
        stream.write_all(&messages::create_connected_message()).unwrap();

    }

    pub fn insert_client_to_room(&mut self, client_id: u8) -> Result<(), ()> {
        let (room_id, is_full) = self.rooms.insert_client_to_room(client_id);
        self.registered_clients.add_client_room(client_id, room_id);

        println!("Client id {} added to room {}", client_id, room_id);
        let payload = messages::create_room_added_message(room_id);
        let _ = &self.registered_clients.clients.get(&client_id).unwrap().socket.as_ref().unwrap().write_all(&payload);

        if is_full {
            self.start_room(room_id);
        }

        Ok(())
    }

    fn start_room(&mut self, room_id: u8) {
        let payload = messages::create_room_started_message();
        // Split participants across different sub-rooms.
        let clients = &self.rooms.rooms.get(&room_id).unwrap().participants_in_room;
        for client_id in clients {
            println!("sending message to client id {}", client_id);
            let _ = &self.registered_clients.clients.get(client_id).unwrap().socket.as_ref().unwrap().write_all(&payload);
        }
        println!("room started");
    }

    pub fn send_message_from_client(&mut self, client_id: u8, msg: String) {
        let room_id = self.registered_clients.clients.get(&client_id).unwrap().room_id;
        if room_id.is_none() {
            return;
        }
        let room = self.rooms.rooms.get_mut(&room_id.unwrap()).unwrap();
        let client_id_recv = room.get_client_id_to_chat(client_id);
        println!("client {}", client_id_recv);

        let payload = messages::create_client_message(msg);
        let _ = &self.registered_clients.clients.get(&client_id_recv).unwrap().socket.as_ref().unwrap().write_all(&payload);
    }
 
}