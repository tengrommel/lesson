use std::rc::Rc;

struct UiucStrudent {
    name: String,
    uin: u64,
    in196: bool,
    gpa: f64
}

fn new_student(name: String,uin: u64) -> UiucStrudent {
    UiucStrudent{
        name,
        uin,
        in196: false,
        gpa: 4.0
    }
}

fn main() {
    struct TestScores(i32, i32, i32);
    struct LectureReviews(i32, i32, i32);

    let test_scores = TestScores(10, 20, 30);
    let lecture_reviews = LectureReviews(5, 5, 5);

    println!("{}", test_scores.0);
    println!("{}", lecture_reviews.2);

    let revers_me = String::from("reverse me");
    let word1 = &revers_me[0..7];
    let word2 = &revers_me[8..10];
    println!("{} {}", word2, word1);

    let number = 11;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2|3|5|7|11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let tripe = (0, -2, 3);
    println!("Tell me about {:?}", tripe);
    match tripe {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
    let number = 15;
    match number {
        x if x % 5 == 0 && x % 3 == 0 => println!("FizzBuzz"),
        x if x % 3 == 0 => println!("Fizz"),
        x if x % 5 == 0 => println!("Buzz"),
        _ => println!("{}", number)
    }

    let mut v = vec![1, 3, 4];
    v.push(1);
    v.push(3);
    v.push(4);
    v.push(5);

    let v = vec![1, 9, 6];
    let second = v[1];
    println!("The second element is {}", second);
    println!("{:?}", v);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    match v.get(199) {
        Some(third) => println!("the hundredth element is {}", third),
        None => println!("There is no hundredth element"),
    }

    let v = vec![1, 9, 6];
    for val in v.iter() {
        println!("{}", val);
    }
    for val in &v {
        println!("{}", val);
    }

    let b = Box::new(5);
    println!("b = {}", b);

    struct DoubleNode<T> {
        data: T,
        next: Option<Rc<DoubleNode<T>>>,
        prev: Option<Rc<DoubleNode<T>>>,
    }
}
