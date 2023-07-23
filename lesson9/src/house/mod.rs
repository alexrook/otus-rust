use crate::providers::*;

pub struct SmartHouse<T>
where
    T: DeviceInfoProvider,
{
    provider: T,
}

impl<T: DeviceInfoProvider> SmartHouse<T> {
    pub fn new(provider: T) -> Self {
        Self { provider }
    }

    pub fn create_report(&self) -> String {
        let mut ret = String::from("This is a house with rooms and devices:\n");
        for room in self.get_rooms() {
            ret = format!("{} the room[{}]:\n", ret, room);
            for device in self.devices(&room) {
                let device_info = self.provider.get_device_info(&room, &device);
                ret = match device_info {
                    Ok(di) => format!("{} \tdevice[{}] with info[{}]\n", ret, device, di),
                    Err(e) => format!(
                        "{} \tsomething went wrong[{}] for device[{}]\n",
                        ret, e, device
                    ),
                }
            }
        }

        ret
    }

    fn get_rooms(&self) -> Vec<String> {
        self.provider.get_rooms()
    }

    fn devices(&self, room: &str) -> Vec<String> {
        self.provider.get_devices(room)
    }
}

#[cfg(test)]
mod test {
    use crate::devices::*;
    use crate::providers::*;

    use super::SmartHouse;

    #[test]
    fn create_smart_house() {
        let ins = SmartHouse {
            provider: OwningDeviceInfoProvider {
                socket: SmartSocket {
                    socket_type: "euro",
                },
            },
        };

        assert!(ins.devices("room1").contains(&"socket".to_string()));
    }
}
