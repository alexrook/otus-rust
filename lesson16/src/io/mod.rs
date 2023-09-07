use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub room: String,
    pub device_name: String,
    pub device_type: String,
    pub cons_arg: String,
}
