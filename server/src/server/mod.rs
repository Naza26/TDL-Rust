pub mod room;

use std::net::TcpStream;
use std::{
    thread::JoinHandle,
};
use std::collections::HashMap;
use std::io::Write;
use crate::client::{ClientInfo, Clients};
use crate::commons::messages;
use crate::server::room::{Rooms, RoomState};



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
        let clients = &self.rooms.rooms.get(&room_id).unwrap().participants_in_room;
        for client_id in clients {
            let _ = &self.registered_clients.clients.get(client_id).unwrap().socket.as_ref().unwrap().write_all(&payload);
        }
        println!("Room {} started", room_id);
    }

    pub fn finish_chat_room(&mut self, client_id: u8) {
        // todo: agregar logica de que finaliza la salita de chat

        let room_id = self.registered_clients.clients.get(&client_id).unwrap().room_id;
        if room_id.is_none() {
            return;
        }

        // finish chat room
        let mut rooms = &mut self.rooms.rooms;
        let client_recv = rooms.get_mut(&room_id.unwrap()).unwrap().get_client_id_to_chat(client_id);

        let payload = messages::create_quit_chatting_message();
        let _ = &self.registered_clients.clients.get(&client_id).unwrap().socket.as_ref().unwrap().write_all(&payload);
        let _ = &self.registered_clients.clients.get(&client_recv).unwrap().socket.as_ref().unwrap().write_all(&payload);

        // finish room
        let should_finish = self.rooms.rooms.get(&room_id.unwrap()).unwrap().should_finish_chat();

        if should_finish {
            self.finish_room(room_id.unwrap());
        }
    }

    fn finish_room(&mut self, room_id: u8) {
        self.rooms.rooms.get_mut(&room_id).unwrap().state = RoomState::ENDED;

        let clients = &self.rooms.rooms.get(&room_id).unwrap().participants_in_room;
        for client_id in clients {
            println!("Sending message to client id {} for finishing room", client_id);
            let list_participants = self.rooms.rooms.get(&room_id).unwrap().get_rest_of_participants(client_id.clone());
            let payload = messages::create_list_participants_message(self.create_participants_hashmap(list_participants));
            let _ = &self.registered_clients.clients.get(client_id).unwrap().socket.as_ref().unwrap().write_all(&payload);
        }
    }

    fn create_participants_hashmap(&self, participants_id: Vec<u8>) -> HashMap<u8, String> {
        let mut participants = HashMap::new();
        for participant_id in participants_id {
            let name = self.registered_clients.get_client_name(participant_id);
            participants.insert(participant_id, name);
        }
        participants
    }

    pub fn send_message_from_client(&mut self, client_id: u8, msg: String) {
        let room_id = self.registered_clients.clients.get(&client_id).unwrap().room_id;
        if room_id.is_none() {
            return;
        }

        let mut rooms = &mut self.rooms.rooms;
        let client_recv = rooms.get_mut(&room_id.unwrap()).unwrap().get_client_id_to_chat(client_id);
        println!("client {}", client_recv);

        let payload = messages::create_client_message(msg);
        let _ = &self.registered_clients.clients.get(&client_recv).unwrap().socket.as_ref().unwrap().write_all(&payload);
    }

    pub fn choose_participant_from_client(&mut self, client_id: u8, participants: Vec<u8>) {
        println!("Client {} chose participants: {:?}", client_id, &participants);
        let payload = messages::create_participants_chosen_message();
        let _ = &self.registered_clients.clients.get(&client_id).unwrap().socket.as_ref().unwrap().write_all(&payload);

        let room_id = self.registered_clients.clients.get(&client_id).unwrap().room_id;
        if let Some(room_id) = room_id {
            self.rooms.add_client_choice_in_room(room_id, client_id, participants);

            if self.rooms.rooms.get(&room_id).unwrap().should_start_matching() {
                println!("Start matching in room {}", room_id);
                self.start_matching(room_id);
            }
        }
    }

    fn start_matching(&self, room_id: u8) {
        let matches = self.rooms.rooms.get(&room_id).unwrap().start_matching();
        for (client_id, client_matches) in matches {
            let payload = messages::create_matching_result_message(client_matches);
            let _ = &self.registered_clients.clients.get(&client_id).unwrap().socket.as_ref().unwrap().write_all(&payload);
        }
    }
}