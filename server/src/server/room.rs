use std::collections::HashMap;
const CAPACITY: u8 = 6;

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

    pub fn insert_client_to_room(&mut self, client_id: u8) -> u8 {
        for (id, room) in &mut self.rooms {
            if !room.is_full() {
                room.add_client(client_id).unwrap();
                return id.clone();
            }
        }

        let id: u8 = self.next_id;
        let mut new_room = Room::new(id, CAPACITY).unwrap();
        new_room.add_client(client_id).unwrap();
        self.rooms.insert(id, new_room);
        self.next_id += 1;

        id
    }
}


#[derive(Debug)]
pub struct Room {
    pub id: u8,
    pub participants: Vec<u8>,
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
            participants: Vec::new(),
            capacity,
            state: RoomState::WAITING
        })
    }

    pub fn add_client(
        &mut self,
        client_id: u8
    ) -> Result<(), ()> {
        if self.participants.len() as u8 >= self.capacity {
            return Err(());
        }
        self.participants.push(client_id);
        Ok(())
    }

    pub fn is_full(&self) -> bool {
        self.participants.len() as u8 == self.capacity
    }
}