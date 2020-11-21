fn main() {
    let array_1 = ['a', 'b', 'c', 'd'];
    let array_2: [u32; 5] = [1,2,3,4,5];
    let array_3 = ['你'; 5];
    println!("array_1: {:?}", array_1);
    println!("array_2: {:?}", array_2);
    println!("array_3: {:?}", array_3);
    let mut x = 0;
    let var = loop {
        if x >= 10 {
            break x * 10;
        }
        x += 1;
    };
    println!("{}", var);
    for var in array_1.iter() {
        println!("{}", var);
    }
}
