// ***** реализация умного дома опираясь на dyn object

use std::collections::HashMap;

// Пользовательские устройства:
struct SmartSocket {
    socket_type: String,
}
struct SmartThermometer {
    measurement_unit: String,
}

trait Device {
    fn get_divice_info(&self) -> String;
}

impl Device for SmartSocket {
    fn get_divice_info(&self) -> String {
        format!("smart socket of type:{}", self.socket_type)
    }
}

impl Device for SmartThermometer {
    fn get_divice_info(&self) -> String {
        format!(
            "smart thermometer with measurement unit:{}",
            self.measurement_unit
        )
    }
}

struct Room<'a> {
    devices: HashMap<String, &'a dyn Device>,
}

impl<'a> Room<'a> {
    fn new() -> Self {
        Room {
            devices: HashMap::new(),
        }
    }

    fn add_device(&mut self, key: String, device: &'a dyn Device) -> &mut Self {
        self.devices.insert(key, device);
        self
    }

    fn show(&self) {
        for (key, device) in self.devices.iter() {
            let info = device.get_divice_info();
            println!("Device[id={},device info={}]", key, info);
        }
    }

    fn create_report(&self) -> String {
        let mut str = self
            .devices
            .iter()
            .map(|(_k, device)| device.get_divice_info())
            .fold("Devices:".to_string(), |acc, info| acc + &info + ",");

        str.pop();
        str
    }
}

struct SmartHouse<'a> {
    rooms: HashMap<String, &'a Room<'a>>,
}

impl<'a> SmartHouse<'a> {
    fn new() -> Self {
        SmartHouse {
            rooms: HashMap::new(),
        }
    }

    fn add_room(&mut self, key: String, room: &'a Room) -> &mut Self {
        self.rooms.insert(key, room);
        self
    }

    fn get_rooms_ids(&self) -> Vec<&String> {
        self.rooms.keys().map(|k| k).collect()
    }

    fn all_devices(&self) -> Vec<&String> {
        let rooms: Vec<&Room<'a>> = self.rooms.iter().map(|(_, room)| room.to_owned()).collect();

        rooms
            .iter()
            .flat_map(|room| room.devices.iter().map(|(deviceId, _)| deviceId))
            .collect()
    }

    fn devices(&self, roomId: &str) -> Vec<&String> {
        let r = match self.rooms.get(roomId) {
            Some(&room) => room.devices.iter().map(|(deviceId, _)| deviceId).collect(),
            _ => Vec::new(),
        };

        r
    }

    fn create_report(&self) -> String {
        self.rooms.iter().fold("".to_string(), |acc, (_, room)| {
            acc + "room[" + &room.create_report() + "]\n"
        })
    }

    fn show(&self) {
        for (_, room) in self.rooms.iter() {
            for (_, device) in room.devices.iter() {
                let info = device.get_divice_info();
                println!("Device:{}", info)
            }
        }
    }
}

// ***** Пример использования библиотеки умный дом:
fn main() {
    //Room1
    let mut room1 = Room::new();
    let socket1: SmartSocket = SmartSocket {
        socket_type: "euro".to_string(),
    };

    let term1: SmartThermometer = SmartThermometer {
        measurement_unit: "Celsius".to_string(),
    };

    room1
        .add_device("euro-soket1".to_string(), &socket1)
        .add_device("smart-term1".to_string(), &term1);

    //Room2
    let mut room2 = Room::new();
    let socket2: SmartSocket = SmartSocket {
        socket_type: "Type A".to_string(),
    };

    let term2: SmartThermometer = SmartThermometer {
        measurement_unit: "Fahrenheit".to_string(),
    };

    room2
        .add_device("type-a-soket1".to_string(), &socket2)
        .add_device("smart-term2".to_string(), &term2);

    // room2.show();

    //Smart House

    let mut smart_house = SmartHouse::new();

    smart_house.add_room("room1".to_string(), &room1);
    smart_house.add_room("room2".to_string(), &room2);

    let report = smart_house.create_report();

    println!("A smart house report:\n{}", report);
}
