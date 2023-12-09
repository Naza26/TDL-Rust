use std::collections::HashMap;
const CAPACITY: u8 = 4;

#[derive(Debug)]
pub enum RoomState {
    Waiting,
    Started,
    Ended
}

#[derive(Debug)]
pub struct Rooms {
    pub rooms: HashMap<u8, Room>,
    pub next_id: u8
}

impl Rooms {
    pub fn new() -> Rooms {
        Rooms {
            rooms: HashMap::new(),
            next_id: 0
        }
    }

    pub fn insert_client_to_room(&mut self, client_id: u8) -> (u8, bool) {
        for (id, room) in &mut self.rooms {
            if !room.is_full() && !room.client_has_chatted_with_everyone_in_the_room(client_id) {
                let is_full = room.add_client(client_id).unwrap();
                return (*id, is_full);
            }
        }

        let id: u8 = self.next_id;
        let mut new_room = Room::new(id, CAPACITY).unwrap();
        new_room.add_client(client_id).unwrap();
        self.rooms.insert(id, new_room);
        self.next_id += 1;

        (id, false)
    }

    pub fn add_client_choice_in_room(&mut self, room_id: u8, client_id: u8, participants: Vec<u8>) {
        let room = self.rooms.get_mut(&room_id).unwrap();
        room.add_client_choice(client_id, participants);
    }
}


#[derive(Debug)]
pub struct Room {
    pub id: u8,
    pub participants_in_room: Vec<u8>,
    pub history_of_chats: HashMap<u8, Vec<u8>>,
    pub participants_chatting: HashMap<u8, Option<u8>>,
    pub capacity: u8,
    pub state: RoomState,
    pub clients_choice: HashMap<u8, Vec<u8>>
}

impl Room {
    pub fn new(
        id: u8,
        capacity: u8,
    ) -> Result<Room, ()> {
        Ok(Room {
            id,
            participants_in_room: Vec::new(),
            history_of_chats: HashMap::new(),
            participants_chatting: HashMap::new(),
            capacity,
            state: RoomState::Waiting,
            clients_choice: HashMap::new()
        })
    }

    // returns if room is full
    pub fn add_client(
        &mut self,
        client_id: u8
    ) -> Result<bool, ()> {
        if self.participants_in_room.len() as u8 >= self.capacity {
            return Err(());
        }
        self.participants_in_room.push(client_id);
        self.history_of_chats.insert(client_id, Vec::new());
        self.participants_chatting.insert(client_id, None);

        if self.is_full() {
            self.state = RoomState::Started;
            return Ok(true);
        }
        Ok(false)
    }

    pub fn get_client_id_to_chat(&mut self, sender_client_id: u8) -> Option<u8> {
        println!("participants chatting: {:?}", self.history_of_chats);
        println!("participants in room: {:?}", self.participants_in_room);
        println!("sender client id: {}", sender_client_id);
        if self.participants_chatting.get(&sender_client_id).unwrap().is_some() {
            return Some(self.participants_chatting.get(&sender_client_id).unwrap().unwrap());
        }

        let mut found_client_id = None;

        // Explicar ownership y problemas

        for (client_id, chatted_participants) in &self.history_of_chats {
            if !chatted_participants.contains(&sender_client_id)
                && *client_id != sender_client_id
                && self.participants_chatting.get(client_id).unwrap().is_none() {
                found_client_id = Some(*client_id);
                break;
            }
        }

        found_client_id?;

        let client_2_chat = self.participants_chatting.get_mut(&found_client_id.unwrap()).unwrap();
        *client_2_chat = Some(sender_client_id);
        let client_1_chat = self.participants_chatting.get_mut(&sender_client_id).unwrap();
        *client_1_chat = Some(found_client_id.unwrap());
        let client_2_history = self.history_of_chats.get_mut(&found_client_id.unwrap()).unwrap();
        client_2_history.push(sender_client_id);
        let client_1_history = self.history_of_chats.get_mut(&sender_client_id).unwrap();
        client_1_history.push(found_client_id.unwrap());

        found_client_id
    }


    pub fn client_has_chatted_with_everyone_in_the_room(&self, client_id: u8) -> bool {
        let participants_chatting = self.history_of_chats.get(&client_id);
        // Explicar Results
        if let Some(participants_chatting) = participants_chatting {
            if participants_chatting.len() as u8 == (self.capacity - 1) {
                return true
            }
        } else {
            return false
        }
        false
    }


    pub fn get_rest_of_participants(&self, client_id: u8) -> Vec<u8> {
        let mut participants = Vec::new();
        for participant in &self.participants_in_room {
            if *participant != client_id {
                participants.push(*participant);
            }
        }
        participants
    }

    pub fn should_finish_room(&self) -> bool {
        for participants in self.history_of_chats.values() {
            if (participants.len() as u8) < self.capacity - 1 {
                return false
            }
        }
        for chatting_client_id in self.participants_chatting.values() {
            if chatting_client_id.is_some() {
                return false
            }
        }
        true
    }

    pub fn is_full(&self) -> bool {
        self.participants_in_room.len() as u8 == self.capacity
    }

    pub fn add_client_choice(&mut self, client_id: u8, participants: Vec<u8>) {
        self.clients_choice.insert(client_id, participants);
        println!("{:?}", &self.clients_choice);
    }

    pub fn should_start_matching(&self) -> bool {
        if self.clients_choice.len() as u8 == self.capacity {
            return true
        }
        false
    }

    pub fn start_matching(&self) -> HashMap<u8, Vec<u8>> {
        let mut choice = HashMap::new();

        for (client_id, participants) in &self.clients_choice {
            let mut client_match = Vec::new();
            for participant in participants {
                if self.is_client_choice(*client_id, *participant) {
                    client_match.push(*participant);
                }
            }
            choice.insert(*client_id, client_match);
        }

        choice
    }

    fn is_client_choice(&self, client_id: u8, client_to_match: u8) -> bool {
        let client_to_match_choices = self.clients_choice.get(&client_to_match).unwrap();
        for choice in client_to_match_choices {
            if choice == &client_id {
                return true
            }
        }
        false
    }

    pub fn finish_chat(&mut self, client_id: u8, client_recv: u8) {
        let value_first_client = self.participants_chatting.get_mut(&client_id).unwrap();
        *value_first_client = None;
        let value_second_client = self.participants_chatting.get_mut(&client_recv).unwrap();
        *value_second_client = None;
    }
}