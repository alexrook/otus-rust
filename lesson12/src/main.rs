mod device;
mod error;
mod provider;

// ***** Пример библиотеки "Умный дом"

use device::*;
use error::*;
use provider::*;

struct SmartHouse {
    rooms: [&'static str; 2],
}

impl SmartHouse {
    fn new() -> Self {
        Self {
            rooms: ["room1", "room2"],
        }
    }

    fn get_rooms(&self) -> [&str; 2] {
        self.rooms
    }

    fn device_name(&self, room: &str) -> Result<String, SmartHouseError> {
        match room {
            "room1" => Ok("socket1".to_string()),
            "room2" => Ok("therm1".to_string()),
            other => Err(SmartHouseError {
                msg: format!("This house doensn't have a room with name[{}]", other),
            }),
        }
    }

    fn get_device<'a, T: DeviceInfoProvider>(
        &self,
        name: &str,
        provider: &'a T,
    ) -> Result<&'a dyn Device, SmartHouseError> {
        let prefix = &name[..3];
        match prefix {
            "soc" => provider.get_soket(),
            "the" => provider.get_term(),
            other => Err(SmartHouseError {
                msg: format!("unknown device type[{}]", other),
            }),
        }
    }

    fn get_device_info<T: DeviceInfoProvider>(&self, room: &str, provider: &T) -> String {
        match self.device_name(room) {
            Err(err) => format!(
                "An error[{}] occured while getting device name for room[{}]",
                err, room
            ),
            Ok(d_name) => match self.get_device(&d_name, provider) {
                Err(err) => format!(
                    "An error[{}] occured while getting device for name[{}]",
                    err, room
                ),
                Ok(device) => device.get_info(),
            },
        }
    }
    fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        self.get_rooms()
            .iter()
            .map(|room| {
                let d_info = self.get_device_info(room, provider);
                format!("A room[{}] with device[{}]\n", room, d_info)
            })
            .reduce(|left, right| left + right.as_str())
            .unwrap_or("Something is wrong with device provider".to_string())
    }
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {
        socket_type: "euro",
    };

    let socket2 = SmartSocket {
        socket_type: "japanese",
    };

    let thermo = SmartThermometer {
        term_type: "Celsium",
    };

    // Инициализация дома
    let house = SmartHouse::new();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };

    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
