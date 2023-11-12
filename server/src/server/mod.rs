pub mod room;

use std::net::TcpStream;
use std::{
    thread::JoinHandle,
};
use crate::client::{ClientInfo, Clients};
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

    pub fn add_client(&mut self, client_id: u8, client_info: ClientInfo, stream: TcpStream) {
        self.registered_clients.add_client(client_id, client_info, stream);
    }

    pub fn insert_client_to_room(&mut self, client_id: u8) -> Result<(), ()> {
        let room_id = self.rooms.insert_client_to_room(client_id);
        println!("Client id {} added to room {}", client_id, room_id);
        Ok(())
    }

 
}