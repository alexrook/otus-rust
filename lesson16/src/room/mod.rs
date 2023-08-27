use std::collections::HashMap;

use crate::device::*;

pub struct Room {
    pub devices: HashMap<String, Box<dyn Device>>,
}

impl Room {
    pub fn new() -> Room {
        Room {
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, name: String, device: Box<dyn Device>) -> &mut Self {
        self.devices.insert(name, device);
        self
    }
    
}
