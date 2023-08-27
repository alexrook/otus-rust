use std::collections::HashMap;

use crate::device::*;
use crate::room::*;

pub struct House {
    pub rooms: HashMap<String, Box<Room>>,
}

impl House {
    pub fn new() -> House {
        House {
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, name: String, room: Room) -> &mut Self {
        self.rooms.insert(name, Box::new(room));
        self
    }

    pub fn upsert_device(
        &mut self,
        room_name: String,
        device_name: String,
        device: Box<dyn Device>,
    ) -> &mut Self {
        if let Some(room) = self.rooms.get_mut(&room_name) {
            room.add_device(device_name, device);
        } else {
            let mut empty_room = Box::new(Room::new());
            empty_room.add_device(device_name, device);
            self.rooms.insert(room_name.clone(), empty_room);
        }
        self
    }

    pub fn report(&self) -> String {
        let mut ret = String::new();
        for (room_name, room) in &self.rooms {
            ret.push_str(&format!("{}:\n", room_name));
            for (device_name, device) in &room.devices {
                let info = device.info();
                ret.push_str(&format!("{}:{}", device_name, info));
                ret.push_str("\n");
            }
            ret.push_str("\n");
        }
        ret
    }

    pub fn get_device_info(
        &self,
        room_name: String,
        device_name: String,
    ) -> Result<String, String> {
        let room = self
            .rooms
            .get(&room_name)
            .ok_or(format!("The room[{room_name}] not found"))?;
        let device: &Box<dyn Device> = room
            .devices
            .get(&device_name)
            .ok_or(format!("The device[{device_name}] not found"))?;
        Ok(device.info())
    }
}
