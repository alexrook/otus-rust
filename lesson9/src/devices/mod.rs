// Пользовательские устройства:
pub struct SmartSocket<'a> {
    pub socket_type: &'a str,
}

pub struct SmartThermometer<'a> {
    pub measurement_unit: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_socket() {
        let s = SmartSocket {
            socket_type: "american",
        };
        assert_eq!(s.socket_type, "american")
    }

    #[test]
    fn create_termometer() {
        let t = SmartThermometer {
            measurement_unit: "C",
        };
        assert_eq!(t.measurement_unit, "C");
    }
}
