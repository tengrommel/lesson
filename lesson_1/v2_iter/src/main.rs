fn main() {
    let mut n = 0;
    loop {
        n += 1;
        if n > 10 {
            break;
        }
        println!("Hello, {}", n);
    }
    println!("All done");
}
