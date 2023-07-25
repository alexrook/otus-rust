use std::fmt;

pub trait Device {
    fn get_info(&self) -> String;
}

pub struct SmartSocket {
    pub socket_type: &'static str,
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = format!("this is a socket whith socket type[{}]", self.socket_type);
        f.write_str(&msg)
    }
}

impl Device for SmartSocket {
    fn get_info(&self) -> String {
        format!("{}", self)
    }
}

pub struct SmartThermometer {
    pub term_type: &'static str,
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = format!(
            "this is a smart thermometer whith socket type[{}]",
            self.term_type
        );
        f.write_str(&msg)
    }
}

impl Device for SmartThermometer {
    fn get_info(&self) -> String {
        format!("{}", self)
    }
}
