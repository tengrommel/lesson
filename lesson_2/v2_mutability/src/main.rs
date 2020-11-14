#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32
}

fn main() {
    let mut x = 34;
    let y = x;
    x += 5;
    println!("y = {}, x = {}", y, x);
    let p = Person {
        name: "Mat".to_string(),
        age: 35,
    };
    let p2 = p;
    println!("p = {:?}, p2 = {:?}", p, p2);
}
