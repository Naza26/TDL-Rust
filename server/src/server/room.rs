use std::collections::HashMap;
const CAPACITY: u8 = 10;

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
        for (id, mut room) in self.rooms {
            if !room.is_full() && !self.client_has_chatted_with_everyone_in_the_room(client_id, &id) {
                let is_full = room.add_client(client_id).unwrap();
                return (id.clone(), is_full);
            }
        }

        let id: u8 = self.next_id;
        let mut new_room = Room::new(id, CAPACITY).unwrap();
        new_room.add_client(client_id).unwrap();
        self.rooms.insert(id, new_room);
        self.next_id += 1;

        (id, false)
    }

    fn client_has_chatted_with_everyone_in_the_room(&self, client_id: u8, room_id: &u8) -> bool {
        let room = self.rooms.get(room_id).unwrap();
        let participants_chatting = room.participants_chatting.get(&client_id).unwrap();
        participants_chatting.len() as u8 == room.capacity
    }
}


#[derive(Debug)]
pub struct Room {
    pub id: u8,
    pub participants_in_room: Vec<u8>,
    pub participants_chatting: HashMap<u8, Vec<u8>>,
    pub capacity: u8,
    pub state: RoomState
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
            state: RoomState::WAITING
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
            if !participants_chatting.contains(&client_id) && client_id != sender_client_id {
                return client_id.clone();
            }
        }
        255
    }


    pub fn is_full(&self) -> bool {
        self.participants_in_room.len() as u8 == self.capacity
    }
}