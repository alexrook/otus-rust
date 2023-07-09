#[derive(Debug)]
struct Test {
    a: String,
    b: String,
}
fn main() {
    let t1 = Test {
        a: "AAA".to_string(),
        b: "BBB".to_string(),
    };
    let t2 = Test {
        a: "CCC".to_string(),
        b: "DDD".to_string(),
    };
    
    let ret =create_vec(t1, t2);


    println!("{:?}",ret)
}

fn create_vec(a: Test, b: Test) -> Vec<Test> {
   vec![a, b]
    
}
