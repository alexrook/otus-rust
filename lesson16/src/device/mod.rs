pub trait Device {
    fn info(&self) -> String;
}

pub struct Poweroutlet {
    pub outlet_type: String,
}

impl Poweroutlet {
    pub fn new(outlet_type: String) -> Poweroutlet {
        Poweroutlet { outlet_type }
    }
}

impl Device for Poweroutlet {
    fn info(&self) -> String {
        format!("This is a [{}] power outlet", self.outlet_type)
    }
}

pub struct Thermometer {
    pub term_type: String,
}

impl Thermometer {
    pub fn new(term_type: String) -> Thermometer {
        Thermometer { term_type }
    }
}

impl Device for Thermometer {
    fn info(&self) -> String {
        format!("This is a [{}] type thermometer", self.term_type)
    }
}
