fn main() {
    let x = 3;
    let v = a(x);
    println!("v is {}", v);
    println!("v is {}", x);
}

fn a(mut n: i32) -> i32 {
    n += 2;
    let res = b(n);
    return res;
}

fn b(n: i32) -> i32 {
    return n*2;
}
