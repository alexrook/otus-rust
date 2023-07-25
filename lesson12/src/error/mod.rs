use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SmartHouseError {
    pub msg: String,
}

impl fmt::Display for SmartHouseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: String = format!("An error[{}] occured while smart house access", self.msg);
        f.write_str(&msg)
    }
}

impl Error for SmartHouseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}
