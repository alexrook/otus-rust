pub trait Device {
    fn info(&self) -> String;
}

pub struct Poweroutlet {
    pub oulet_type: String,
}

impl Poweroutlet {
  pub  fn new(outlet_type: String) -> Poweroutlet {
        Poweroutlet {
            oulet_type: outlet_type,
        }
    }
}

impl Device for Poweroutlet {
    fn info(&self) -> String {
        format!("This is power outlet with type[{}]", self.oulet_type)
    }
}

pub struct Thermometer {
    pub term_type: String,
}

impl Thermometer {
    pub fn new(term_type: String) -> Thermometer {
        Thermometer {
            term_type: term_type,
        }
    }
}

impl Device for Thermometer {
    fn info(&self) -> String {
        format!("This is termometer with type[{}]", self.term_type)
    }
}
