#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

fn main() {
    let a = divide(4, 5);
    let b = divide(10, 0);
    // match a {
    //     Res::Thing(v) => println!("val = {}", v),
    //     _ => {}
    // }
    if let Result::Ok(v) = a {
        println!("val = {}", v)
    }
    println!("a = {:?}, b = {:?}", a, b);
}

fn divide(a: i32, b:i32) -> Result<i32, String> {
    if b == 0 {
        return Result::Err("Cannot Divide by zero".to_string());
    }
    Result::Ok(a/b)
}
