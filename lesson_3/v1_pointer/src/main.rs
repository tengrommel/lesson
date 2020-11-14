#[derive(Debug)]
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
    pub fn age_up(&mut self, n: i32) {
        self.age += n
    }
    pub fn drop_self(self){}
}

fn main() {
    let mut p = Person::new("matt".to_string(), 35);
    let s = p.greet();
    println!("{}", s);

    let a = get_age(&p);
    println!("person's age is {}", a);

    // p.drop_self();
    println!("person's age is {}", a);
    p.age_up(2);
    // println!("person's age is {}", a);

    let s2 = p.greet();
    println!("really: {}", s2);
    p.age_up(3);
    println!("{:?}", p)
}

fn get_age(s: &Person) -> &i32 {
    &s.age
}
