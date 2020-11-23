fn main() {
    let mut num_to_increment = 5;
    increment_by_five(&mut num_to_increment);
    println!("The number {}", num_to_increment);
}

fn increment_by_five(num: &mut i32) {
   *num += 5;
}
