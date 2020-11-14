pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Person {name, age}
    }
    pub fn greet(&self) -> String {
        format!("Hi my name is {} ", self.name)
    }
}

fn main() {
    let p = Person::new("matt".to_string(), 35);
    let s = p.greet();
    println!("{}", s);
    let s2 = p.greet();
    println!("really: {}", s2);

}
