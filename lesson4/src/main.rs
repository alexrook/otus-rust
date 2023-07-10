mod smart_house {
    use rand::{thread_rng, Rng};
    #[derive(Debug)]
    pub struct SmartSocket {
        state: bool,
        pub description: String,
    }

    impl SmartSocket {
        pub fn turn_off(&mut self) {
            println!("It's turned off");
            self.state = false
        }

        pub fn turn_on(&mut self) {
            println!("It's turned on");
            self.state = true
        }

        pub fn create(desc: &str) -> Self {
            SmartSocket {
                state: false,
                description: String::from(desc),
            }
        }

        pub fn show_state(&self) -> bool {
            self.state
        }
    }

    pub struct Thermometer {
        randomizer: rand::rngs::ThreadRng,
    }

    impl Thermometer {
        pub fn get_celsius(&mut self) -> i32 {
            self.randomizer.gen_range(-50..45)
        }

        pub fn create() -> Self {
            Thermometer {
                randomizer: thread_rng(),
            }
        }
    }
}

fn main() {
    let mut socket1: smart_house::SmartSocket =
        smart_house::SmartSocket::create("My smart socket 1");

    socket1.turn_on();

    socket1.turn_off();

    let state: bool = socket1.show_state();
    println!("socket1.desc[{:?}]", state);

    //Я ожидал что здесь будет ошибка
    //поскольку поле state уже borrow ?
    let state2: bool = socket1.show_state();
    println!("socket1.desc[{:?}]", state2);

    println!("socket1[{:?}]", socket1);
    println!("socket1.desc[{:?}]", socket1.description);

    let mut term1 = smart_house::Thermometer::create();

    println!("Temperature im my house is[{}]", term1.get_celsius());
    println!("Next call get_celsius[{}]", term1.get_celsius())
}
