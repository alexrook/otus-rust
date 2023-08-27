
mod room;
mod device;
mod house;
mod io;

use crate::room::*;
use crate::device::*;
use crate::house::*;
use crate::io::*;



fn main() {
    let file_content = include_str!("resources/rooms_and_devices.json");

    let devices: Vec<DeviceInfo> = serde_json::from_str(file_content).unwrap();


    let mut room1 = Room::new();

    room1
        .add_device(
            "1".to_string(),
            Box::new(Thermometer::new("Fahrenheit".to_string())),
        )
        .add_device(
            "2".to_string(),
            Box::new(Poweroutlet::new("Type A".to_string())),
        )
        .add_device(
            "3".to_string(),
            Box::new(Poweroutlet::new("Type C".to_string())),
        );

    let mut room2 = Room::new();

    room2
        .add_device(
            "room2-term1".to_string(),
            Box::new(Thermometer::new("Celsium".to_string())),
        )
        .add_device(
            "room2-term2".to_string(),
            Box::new(Thermometer::new("Kelvin".to_string())),
        )
        .add_device(
            "room2-po1".to_string(),
            Box::new(Poweroutlet::new("Euro".to_string())),
        )
        .add_device(
            "room2-po2".to_string(),
            Box::new(Poweroutlet::new("Type B".to_string())),
        );

    let mut house = House::new();

    house
        .add_room("Room1".to_string(), room1)
        .add_room("Room2".to_string(), room2);

    house.upsert_device(
        "Room1".to_string(),
        "TV otlet".to_string(),
        Box::new(Poweroutlet::new("Type A".to_string())),
    );

    house.upsert_device(
        "Room3".to_string(),
        "Computer outlet".to_string(),
        Box::new(Poweroutlet::new("Type C".to_string())),
    );

    for device_info in devices {
        match device_info.device_type.as_str() {
            "Thermometer" => {
                house.upsert_device(
                    device_info.room,
                    device_info.device_name,
                    Box::new(Thermometer::new(device_info.cons_arg)),
                );
                ()
            },
            "Poweroutlet"=>{
                house.upsert_device(
                    device_info.room,
                    device_info.device_name,
                    Box::new(Poweroutlet::new(device_info.cons_arg)),
                );
                ()
            }
            other => println!("Unknown device type[{}]", other),
        }
    }

    let report = house.report();
    println!("The house report:\n[\n{}]", report);

    let dev_info1 = house
        .get_device_info("Room1".to_string(), "1".to_string())
        .unwrap();
    println!("Ok:{dev_info1}");

    let dev_info2 = house.get_device_info("No room".to_string(), "not exeistent 1".to_string());

    match dev_info2 {
        Err(e) => println!("Error:{e}"),
        Ok(ret) => println!("Ok:{ret}"),
    }
}
