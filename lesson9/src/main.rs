// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым
pub mod devices;
pub mod house;
pub mod providers;

use devices::*;
use house::*;
use providers::*;
// ***** Пример использования библиотеки умный дом:

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {
        socket_type: "euro",
    };
    let socket2 = SmartSocket {
        socket_type: "japanese",
    };
    let thermo = SmartThermometer {
        measurement_unit: "F",
    };
    // // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    // Инициализация дома
    let house1 = SmartHouse::new(info_provider_1);

    let report1 = house1.create_report();

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let house2 = SmartHouse::new(info_provider_2);

    let report2 = house2.create_report();

    // // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
