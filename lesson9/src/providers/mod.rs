use crate::devices::*;
// Пользовательские поставщики информации об устройствах.
pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, device_name: &str) -> Result<String, String>;
    fn get_devices(&self, room: &str) -> Vec<String>;
    fn get_rooms(&self) -> Vec<String>;
}

// Могут как хранить устройства, так и заимствывать.
pub struct OwningDeviceInfoProvider<'a> {
    pub socket: SmartSocket<'a>,
}

impl<'a> DeviceInfoProvider for OwningDeviceInfoProvider<'a> {
    fn get_device_info(&self, room_name: &str, device_name: &str) -> Result<String, String> {
        match room_name {
            "room1" => match device_name {
                "socket" => Ok(format!(
                    "a socket with socket type[{}]",
                    self.socket.socket_type
                )),
                other => Err(format!(
                    "This device provider doesn't coontain the[{}] device",
                    other
                )),
            },
            other => Err(format!(
                "This device provider doesn't coontain the[{}] room",
                other
            )),
        }
    }

    fn get_rooms(&self) -> Vec<String> {
        vec!["room1".to_string()]
    }

    fn get_devices(&self, room: &str) -> Vec<String> {
        match room {
            "room1" => vec!["socket".to_string()],
            _ => Vec::new(),
        }
    }
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket<'a>,
    pub thermo: &'b SmartThermometer<'b>,
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_info(&self, room_name: &str, device_name: &str) -> Result<String, String> {
        match room_name {
            "room1" => match device_name {
                "socket" => Ok(format!(
                    "a socket with socket type[{}]",
                    self.socket.socket_type
                )),
                other => Err(format!(
                    "This device provider doesn't coontain the[{}] device",
                    other
                )),
            },
            "room2" => match device_name {
                "thermo" => Ok(format!(
                    "a thermometer with measurement unit[{}]",
                    self.thermo.measurement_unit
                )),
                other => Err(format!(
                    "This device provider doesn't coontain the[{}] device",
                    other
                )),
            },

            other => Err(format!(
                "This device provider doesn't coontain the[{}] room",
                other
            )),
        }
    }

    fn get_rooms(&self) -> Vec<String> {
        vec!["room1".to_string(), "room2".to_string()]
    }

    fn get_devices(&self, room: &str) -> Vec<String> {
        match room {
            "room1" => vec!["socket".to_string()],
            "room2" => vec!["thermo".to_string()],
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_owning_provider() {
        let ins = OwningDeviceInfoProvider {
            socket: SmartSocket {
                socket_type: "euro",
            },
        };
        assert_eq!(
            ins.get_devices("room1").contains(&"socket".to_string()),
            true
        )
    }

    #[test]
    fn create_borrowing_provider() {
        let ins = BorrowingDeviceInfoProvider {
            socket: &SmartSocket {
                socket_type: "euro",
            },
            thermo: &SmartThermometer {
                measurement_unit: "F",
            },
        };
        assert_eq!(
            ins.get_devices("room2").contains(&"thermo".to_string()),
            true
        )
    }
}
