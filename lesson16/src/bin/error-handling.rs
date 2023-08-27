use std::error::Error;
use std::fmt::Display;
use std::fs::*;
use std::io::Write;

fn main() {
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    match foo("test.1.txt") {
        Err(e) => println!("Error :{}", e),
        Ok(ret) => println!("Success :{}", ret.metadata().expect("Metadata").is_file()),
    }

    let md = MyData("aaa".to_string());

    let r = bar(&md);

    println!("{}", r);
}

#[derive(Debug)]
struct MyError {
    msg: String,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("An error occured[{}]", self.msg))
    }
}

impl Error for MyError {}

fn foo(name: &str) -> Result<File, MyError> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(name)
        .map_err(|e| MyError {
            msg: format!("An open error occured[{}]", e),
        })?;

    file.write_all(b"My useful bytes\n").map_err(|e| MyError {
        msg: format!("A write error occured[{}], error kind[{}]", e, e.kind()),
    })?;

    Ok(file)
}

struct MyData(String);

fn bar(md: &MyData) -> &str {
    &md.0
}
