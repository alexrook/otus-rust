mod garden;

use garden::do_something;

fn main() {
    let a = "World";
    println!("Hello Rust");
    println!("Hello, {}!", a);
    println!("It Works!");

    do_something(42);
}

 
