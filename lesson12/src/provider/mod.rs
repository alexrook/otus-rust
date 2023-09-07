use crate::device::*;
use crate::error::*;

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.

pub trait DeviceInfoProvider {
    fn get_soket(&self) -> Result<&dyn Device, SmartHouseError>;
    fn get_term(&self) -> Result<&dyn Device, SmartHouseError>;
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}
pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_soket(&self) -> Result<&dyn Device, SmartHouseError> {
        Ok(&self.socket)
    }
    fn get_term(&self) -> Result<&dyn Device, SmartHouseError> {
        Err(SmartHouseError {
            msg: "This provider doens't have termometer".to_string(),
        })
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_soket(&self) -> Result<&dyn Device, SmartHouseError> {
        Ok(self.socket)
    }

    fn get_term(&self) -> Result<&dyn Device, SmartHouseError> {
        Ok(self.thermo)
    }
}
