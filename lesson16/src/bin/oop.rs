#![allow(unused_variables)]

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    use std::f64::consts::PI;
    let answer = 42;
    let maybe_pi = PI;
    let v: Vec<&dyn Show> = vec![&answer, &maybe_pi];
    for d in v.iter() {
        println!("show {}", d.show());
    }
}
