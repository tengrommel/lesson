#[derive(Debug)]
struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl <T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut ll = LinkedList{
        data: 3,
        next: Some(Box::new(LinkedList{data:2, next:None}))
    };
    println!("{:?}", ll);
    if let Some(ref mut v) = ll.next {
        v.add_up(10);
    }
    println!("{:?}", ll);
    let mut v = Vec::new();
    v.push("Hello".to_string());
    v.push("Goodbye".to_string());
    for i in 0..105 {
        v.push(i.to_string());
    }
    println!("v.len = {} v.cap = {}", v.len(), v.capacity());
    let mut s = "   hello   ".to_string();
    let p = s.trim();
    let p = p.to_string();
    s.push_str("goodbye");
    println!("p = '{}'", p);
    let f_str = "help me find home";
    let ff_str = string_stuff_find_f(f_str);
    println!("ff_str = '{}'", ff_str);
    println!("chosen = '{}'", choose_str(1));
}

fn string_stuff_find_f(s: &str) -> &str {
    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..];
        }
    }
    s
}

fn choose_str(n: i32) -> &'static str {
    match n {
        0 => "hello",
        1 => "goodbye",
        _ => "other",
    }
}
