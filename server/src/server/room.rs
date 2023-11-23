use std::collections::HashMap;
const CAPACITY: u8 = 2;

#[derive(Debug)]
pub enum RoomState {
    WAITING,
    STARTED,
    ENDED
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
    pub participants_chatting: HashMap<u8, Vec<u8>>,
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
            participants_chatting: HashMap::new(),
            capacity,
            state: RoomState::WAITING,
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
        self.participants_chatting.insert(client_id, Vec::new());

        if self.is_full() {
            self.state = RoomState::STARTED;
            return Ok(true);
        }
        Ok(false)
    }

    pub fn get_client_id_to_chat(&mut self, sender_client_id: u8) -> u8 {
        for (client_id, participants_chatting) in &self.participants_chatting {
            if !participants_chatting.contains(client_id) && *client_id != sender_client_id {
                return client_id.to_owned();
            }
        }
        return 255;
    }

    pub fn client_has_chatted_with_everyone_in_the_room(&self, client_id: u8) -> bool {
        let participants_chatting = self.participants_chatting.get(&client_id).unwrap();
        participants_chatting.len() as u8 == self.capacity
    }


    pub fn get_rest_of_participants(&self, client_id: u8) -> Vec<u8> {
        let mut participants = Vec::new();
        for participant in &self.participants_in_room {
            if participant.clone() != client_id {
                participants.push(participant.clone());
            }
        }
        return participants;
    }

    pub fn should_finish_chat(&self) -> bool {
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
                if self.is_client_choice(client_id.clone(), participant.clone()) {
                    client_match.push(participant.clone());
                }
            }
            choice.insert(client_id.clone(), client_match);
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
}