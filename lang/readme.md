# 1
# Your First Program! -setup, files and stuff

- Rust files end with .rs
- To compile a rust file run 
    
    
    rustc file_name.rs 

- This generates an executable
- You can then run executable

# Your First Program! -main
- Quick (very quick)recap:
    - Functions are a named chunk of code that does a specific job
- The main function is unique because it will always run first

    
    fn main() {
    }
- 'fn' identifies(确定) the function, with 'main' being the function name 
- The parentheses '()' are where we would place parameters
    - As we can see we have none at the moment
- The function is surrounded by curly braces


    fn main() {
        println!("Hello, world");
    }
- We use the command "println!" to print our message
- Note: the "!".This means that we are using a Rust macro
- A function call would not use the "!"
- We pass the string "Hello World" into the macro as an argument
- End the line with a semicolon;
- (small thing to note is that Rust style uses 4 spaces, not
 a tab)
 
 # cargo
 
 - Cargo is Rust's build system and package manager
 - Cargo can do a lot of things like:
    - Build your code
    - Downloading libraries that your program is using
    - Building libraries
 - Sounds great! How do I get it?
    - Cargo be installed with Rust
 - Check it 
    
    
    cargo --version

# Cargo - Creating a Project
To create a project using cargo you just write "cargo new <project_name>"
- Cargo creates a directory with called src(source code) with a file main.rs inside
- Cargo creates a file called Cargo.toml - this file has information on the project details
- Cargo initializes a Git repository and a .gitignore file

# Cargo - Building and Running
- build using cargo you write cargo build in the working directory
- This will create a target directory with a debug directory inside it. The debug directory inside it. The debug directory has our executable
- To run the executable we can use ./target/debug/<executable_name> or use cd target/debug && ./<executable_name>
- Using cargo run in the working directory to compile and run the executable in one line
- Cargo creates a file called Cargo.lock which keeps track of libraries(in the textbook they are referred to as dependencies)
- You don't need to worry about this as Cargo automatically updates it

# Cargo - Check
- The command cargo check will check if your code can compile but won't create an executable
- Why would we do this? Building you code is a way to check for errors, but cargo build can be slow. cargo check is a fast way to do this.

# Cargo - Build for Release
- Once you are finished with your project you can build for release
- Using the command cargo *build -- release* will build your code for release
- The compiler will provide extra optimizations in target/release instead of target/debug
- The optimizations make your code run faster, but takes longer to compile
- When developing code, use cargo build as you will frequently rebuild your code to test it
- Use *cargo build --release* to make a fast, finished executable

# Variables and Mutability
- By default variables are immutable in rust
    - This is done for safety and for safe concurrency


    fn main() {
        let change_me = 5;
        println!("The variable has value {}", change_me);
    } 

- We use the keyword *let* to declare a variable
- Any guesses about what this does?


    fn main() {
        let mut change_me = 5;
        println!("The variable has value {}", change_me);
        change_me = 3;
        println!("The variable has value {}", change_me);
    } 
- you can use mut to make it mutable

# Shadowing

- In Rust you can declare a variable with the same name as a previous variable
- We say the new variable shadows the original variable

    
    fn main() {
        let var = 10;
        println!("The variable has value {}", var);
        let var = var+1;
        println!("The variable has value {}", var);
        let var = var * 2;
        let var = var/4;
        println!("The variable has value {}", var)
    }
- Things to note:
    - Shadowing is different from using mut because we are indicating that we want to make a few transformations to variable but still keep it immutable
    - As we are essentially declaring a new variable we can change the type
    
# 2 
# Data types

- All values in Rust are a specific Data Type
- This allows the language to know how much space to allocate and how to work with the data
- You don't need to specify the type when declaring a variable as the compiler is smart and can infer what data type a variable is 

# Data Types - Scalar Types
- Scalar Types represent a single value
- Rust has 4 primary scalar types:
    - Integers
    - Floating-point numbers
    - Boolean
    - Characters

# Integer
- Integers are whole numbers - i.e. have no decimal part
- We can use signed (positive and negative) and unsigned (positive only) integers
- The number following the letter is the number of bits we use to represent the number
    - i<n>
    - u<n>
    - arch depends on your machine
 
# Floating Points
- Floating point numbers are numbers with a decimal part
- Rust has
    - f32 which uses 32 bit
    - f64 which uses 64 bit
- f64 is the default as it allows for higher precision with similar speed as f32 on modern computers

# Boolean

# Character
- Can hold one Unicode Scalar Value
- We use single quotes to represent a single character, and double quotes for string

# Compound Types
- Compound types can hold multiple values in one type
- Rust has two compound types 
    - Tuples
    - Arrays

# Tuples
- Tuples are a compound type which can hold several values in a single variable 
- We can create a tuple using a comma separated list in parentheses
- Tuples have fixed length
    
    
    let tup_1 = ('a', 2, 42.35);
    let (char_var, int_var, float_var) = tup_1;
    println!("The character (first value) in the tuple is: {}", char_var);
    println!("The integer (second value) in the tuple is: {}", int_var);
    println!("The float (third value) in the tuple is: {}", float_var);
    
# Arrays
- Arrays are another compound data type in Rust
- Arrays are:
    - Fixed length(different from other programming languages)
    - Can only hold values with the same data type
- We can create an array using a comma separated list in square brackets
- We will cover vectors in later lectures which can grow/shrink in size

# Control Flow - Conditional 
- We use conditionals to choose what code to run based on the current value
- (they're just if statements)

    
    if <some_statement> {
        // do something if statement is true
    } else {
        // do something if statement is false
    }
- If statements are written in this form
- Conditions do not need parentheses around them
- The statements are surrounded by curly brackets
- The statements do not need semicolons at the end

You can assign an if statement to a variable
    - E.g.let var = <if_statement>

# Control Flow - loops
- A loop will just repeatedly run a section of code
- You can assign a loop to variable
- You can use the keyword break to leave the loop return the value to a value

# Control Flow - While loops
> A while loop will just repeatedly run a section of code while the condition is true

# Control Flow - For loops
- For loops allow you to iterate over a collection of values
- This is pretty generalized and you can do more with them, but this is a good way to start thinking about them
- The loop will run the chunk of code for length of the collection
- We will cover:
    - The indexed for loop 
    - The for-each loop 
    
Control Flow - For each loops

- Works similarly to how for-each loops work in Python
- The loop will run the chunk of code for length of the collection

    
    for <var> in <collection>.iter() {
        // run code
    }
# Functions - Revisited

    fn add(first_number: i32, second_number: i32) -> i32 {
        first_number + second_number
    } 
    
    fn do_nothing() {
        
    }

# Comments 
*//* 

# Memory Management in Rust and its implications
> Lets build up some ideas before we dive into the Rust Memory Model

- Computers use memory(duh). What does this look like?
> The Stack and the Heap are parts of memory that are available to your code to use at runtime, but they are structured in different ways and hold different types of data.

- The Stack functions just like the data structure, but in this case is used to store data of a known, fixed size
- The Heap is just a general term that describes boxes, but in this case is used to store data of an unknown size or size that might change

Imagine the stack in Rust as a person sitting at a table reading your code with papers coming in.

The stack in Rust holds data of a Fixed size

- 1、Each value in Rust has a variable that's called its owner.
- 2、There can only be one owner at a time.
- 3、When the owner goes out of scope, the value will be dropped.

    
    let x = 4;
    // We make a copy of the value in x and bind it to y.
    let y = x;
    // Because these values are of a fixed size
    
    let s1 = String::from("hello"); // s1 owner of "hello"
    let s2 = s1; // s2 is owner of "hello" s1 is destory
    
    let s1 = String::from("hello"); // s1 is owner of "hello"
    let s2 = s1.clone(); // s2 is new owner of new "hello"

# Why
> Keep ownership in mind

Functions will take ownership of the variables that are passed to unless started otherwise(if the value is stored on the heap)


    fn main() {
        let s = String::from("hello");
        print_string(&s);
        let x = 5;
        print_num(x);
        println!("{}", s);
        println!("{}", x);
    }
    
    fn print_string(some_string: &String) {
        println!("{}", some_string);
    }
    
    fn print_num(some_integer: i32) {
        println!("{}", some_integer);
    }

function can give ownership

    fn main() {
        let s1 = gives_ownership();
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
    }
    
    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        some_string // given the ownership
    }
    
    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

s2已经被销毁了

For one, this can get tedious(having to give the ownership back everytime we want to keep it).

For another, this can get messy.

如果你返回一个元组，这将十分难看

Let's talk about 'Borrowing'

    fn main() {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len); // this will error, s1 is invalid now.
    }
    
    fn calculate_length(s: &String) -> usize {
        let length = s.len();
        length
    }

Borrowing in Rust as 'references'
> If you would like, you can make them mutable, however.

Caveat(警告)
> At any given time, you can have either one mutable reference or any number of immutable references, but not at the same time!

     let mut s = String::from("hello");
     let r1 = &s;
     let r2 = &s;
     let r3 = &mut s; // 错误
     println!("{}, {}, and {}", r1, r2, r3);
# Dereferencing 
> Since the variable holding the reference is not the value itself, rather a pointer to the value, we need to dereference to access the value 

    fn main() {
        let mut num_to_increment = 5;
        increment_by_five(&mut num_to_increment);
        println!("The number {}", num_to_increment);
    }
    
    fn increment_by_five(num: &mut i32) {
       *num += 5;
    }

以前的已经rust已经自动解引用

#3

# Slice Type
- Slice allow us to reference contiguous sequence of elements in a collection, instead of the whole collection
- Slice do not have ownership
- Example of using slices
> Accessing the first word in a string

String Slices
> A string slice is a reference to a part of a string

    let revers_me = String::from("reverse me");
    let word1 = &revers_me[0..7];
    let word2 = &revers_me[8..10];
    println!("{} {}", word2, word1);
A string is essentially an array(list) of characters

- Internally the data structure stores the starting position and the length of the slice(which is the ending index - starting index)

- Using the .. notation we can do a few clever things. Let's say we want to slice a string s
- If we want to go from 0 to <some_index> we can write.
    - s[..<some_index>]
- If we want to go from <some_index> to the end of the string we can write:
    - s[<some_index>..]
- If we want the whole string we can write:
    - s[..]

# String Slices - Final Bits
- The string slice type is written as: &str
# String literals are string slices
- e.g.let literal = "a literal!"

# Structs
> Is a custom data type that allows you to group a set of values that make meaning ful group 

How To Create A Struct:
- We declare a struct by using the keyword "struct" followed by the struct name
- In the struct we define different values to store and the type in a comma separated list

    
    struct <struct_name> {
        <var_1>: <type>,
        <var_2>: <type>
    }

# Declaring A struct

- We use the let keyword
- Within brackets we assign *key: value* pairs
- NOTE: The order of the *key: value* pairs do not have to match the struct declaration

# Accessing Values
> We can access values in the struct using dot notation

    
    student1.name
Similarly if we make the variable mutable, we can edit the values using dot notation

# Using a function to Create a Struct

# A Better way Struct Update Syntax


    let student2 = UiucStudent {
        name: String::from("Two"),
        uin: 123434m
        ..student1
    }
 
# Tuple Structs
- We can also define structs that look like tuples known as... Tuple structs!
- Similar to structs in that they provide meaning by their name
    - i.e.tuple structs for coordinates can be called 'coordinates'
- Main difference is that the fields don't have names - only type
- This is useful when you want to give a tuple structure a name


     struct TestScores(i32, i32, i32);
     struct LectureReviews(i32, i32, i32);
    
     let test_scores = TestScores(10, 20, 30);
     let lecture_reviews = LectureReviews(5, 5, 5);
    
     println!("{}", test_scores.0);
     println!("{}", lecture_reviews.2);
     
 # Match 
 - We can use pattern matching in rust by using the keyword
 - This works like switch statements 
 
    
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

Match Guard
> We can use guards to have more complex cases, using if conditions

# Vectors
- Vectors are a powerful collection data type that let you hold more than one value in a single data structure
- We refer to our vector using the notation Vec<T> where 'T' is a specific type
- Vectors can only store values of the same type
- We create a vector by calling the Vec::new function: 
    
    
    let v: Vec<i32> = Vec::new();
- We specified the type here because this is an empty list and Rust know the type of our vector
- Often when writing code you will create a Vec<T> with initial values and Rust can infer the type from there
- There is a Rust macro, *vec!*, that we use for convenience:
    
    
    let v = vec![1, 9, 6];
    for val in v.iter() {
        println!("{}", val);
     }
     for val in &v {
         println!("{}", val);
     }

# LinkList 

# What is a pointer
> Abstractly, they are an arrow to a value stored somewhere else 

# Our first smart pointer - Box<T>
> Allow us to store data on the heap rather than the stack

- What remains on the stack is the pointer to the heap data.
- Box is a smart pointer because it adds additional features -- it implements 'Deref' -- not super important for today but...
    
    
    let b = Box::new(5);
    println!("b = {}", b);
Box struct

    struct Node<T> {
        data: T, 
        next: Option<Box<Node<T>>>
    }
Doubly Linked Lists
>Same as 'Singly' Linked Lists except instead of just having a next pointer, they have a prev pointer as well

If only we could have multiple owners in Rust...(thinking)

# Rc<T>
- Sometimes, we actually need to be allowed for a value to have multiple owners
- Doubly Linked List is an example of this
- Rc<T> is short for reference counting
- This type keeps track of how many references we have to the value, and then cleans up when we're done!
- This is a smart pointer because of this additional feature -- reference counting


    struct DoubleNode<T> {
        data: T,
        next: Option<Rc<DoubleNode<T>>>,
        prev: Option<Rc<DoubleNode<T>>>
    }    

# RefCell<T>
- Just like the compile-time borrowing rules, RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time.
- RefCell<T> keeps track of how many immutable and mutable borrows we have
- RefCell will 'panic' at runtime if you are caught trying to have more than one mutable borrow instead of not allowing you to compile


    struct DoubleNode<T> {
        data: T,
        next: Option<Rc<DoubleNode<T>>>,
        prev: Option<Rc<DoubleNode<T>>>
    }
    
# Functional Programming
> Functional programming is the process of building software by composing pure functions, avoiding shared state, mutable data, and side-effects. Contrast with object oriented programming, where application state is usually shared and collocated with methods in objects

- Pure Functions: Every input has the same output
- Side Effects: Any application state change that is observable outside the called function other than its return value

*Functional code tends to more concise, more predictable, and easier to test than imperative or object oriented code - but if you're unfamiliar with it and the common pattern associated with it, functional code can also seem a lot more dense, and the related literature can be impenetrable to newcomers.*

# Feature of Functional Programming: First Class Functions
>When functions are treated like any other variable. For example, in such a language, a function can be passed as an argument to other functions, can be returned by another function and can be assigned as a value to a variable

closure
- Anonymous functions you can save in a variable or pass as arguments to other functions
- We refer to the anonymous function using the name of the variable that is referring to it, but that does not make it a named function(more on this later)
- Unlike functions, closures can capture values from the scope in which they're defined 
- To define a closure, we start with a pair of vertical pipes(|), inside which we specify the parameters to the closure
- If we had more than one parameter, we would separate them with commas, like |param1, param2|.
- After the parameters, we place curly brackets that hold the body of the closure -- these are optional if the closure body is a single expression


# map
# filter
# Fold

Unlike functions, closures can capture values from the scope in which they're defined

闭包
>closure expressions语法

    ClosureExpression :
        move?
        ( || | | ClosureParameters? | )
        (Expression | -> TypeNoBounds BlockExpression)
     
     ClosureParameters :
        ClosureParam (, ClosureParam)* ,?
     
     ClosureParam :
        OuterAttribute* Pattern ( : Type )?
 
 一个closure expression生成一个closure type，这个类型是唯一的，匿名的，无法被显式表达的
 
    
     fn f<F : FnOnce() -> String> (g: F) {
         println!("{}", g());
     }
     
     let mut s = String::from("foo");
     let t = String::from("bar");
     
     f(|| {
         s += &*t;
         s
     });
     // Prints "foobar".
     
 等价于
 
     struct Closure<'a> {
         s : String,
         t : &'a String,
     }
     
     impl<'a> FnOnce<()> for Closure<'a> {
         type Output = String;
         fn call_once(self) -> String {
             self.s += &*self.t;
             self.s
         }
     }
closure是可以保存进变量或者作为参数传递给其他函数的匿名函数，也可以作为函数的返回值返回，然后在不同的上下文中执行闭包运算，其执行的逻辑可能在不同环境中都略有不同，还可以捕获调用者作用于中的变量。

# Capture modes
Closure的捕获模式有四种：immutable borrow，unique immutable borrow，mutable borrow，move，编译器会按照顺序选择一个模式，该模式能使代码编译通过。

如果显示使用了move关键字，则不管借用是否能满足闭包的实际要求，所有变量的捕获都会变为move模式，对于实现了Copy trait的类型，则执行copy。move关键字经常被用来允许Closure比捕获变量生命周期更久（closure outlive the captured values），比如当闭包被当作返回值返回并被用于发起一个新线程时。

     struct SetVec {
         set: HashSet<u32>,
         vec: Vec<u32>
     }
     
     impl SetVec {
         fn populate(&mut self) {
             let vec = &mut self.vec;
             self.set.iter().for_each(|&n| {
                 vec.push(n);
             })
         }
     }

# move（移动所有权）

    fn main() {
        let a = String::from("hello");
        let closure = || {a + ",world"};
        //^ value borrowed here after move
        //println!("{:?}", a);
        println!("{:?}", closure());
    }

     fn main() {
         let a = String::from("hello");
         let closure = move || {a + ",world"};
         println!("{:?}", closure());
     }
     
# Fn, FnOnce, FnMut

    pub trait FnOnce<Args> {
         /// The returned type after the call operator is used.
         #[stable(feature = "fn_once_output", since = "1.12.0")]
         type Output;
         /// Performs the call operation.
         #[unstable(feature = "fn_traits", issue = "29625")]
         extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
     }
     pub trait FnMut<Args>: FnOnce<Args> {
         /// Performs the call operation.
         #[unstable(feature = "fn_traits", issue = "29625")]
         extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
     }
     pub trait Fn<Args>: FnMut<Args> {
         /// Performs the call operation.
         #[unstable(feature = "fn_traits", issue = "29625")]
         extern "rust-call" fn call(&self, args: Args) -> Self::Output;
     }
     
- 规则一：所有的Closure都实现了FnOnce，这意味着Closure可以通过消费Closure所有权的方式被调用一次
- 规则二：如果一个Closure没有move out任何捕获的变量，则其实现FnMut ，意味着其可以通过可变引用进行调用
- 规则三：如果一个Closure没有修改或move out任何捕获的变量，则其实现Fn，意味着其可以通过共享引用（不可变引用）进行调用

所有的Closure都实现了Sized trait，闭包还可能自动实现以下类型的trait，具体取决于其捕获了何种类型的变量
- Clone
- Copy
- Sync
- Send

# What is Concurrency

Concurrency: the ability for a program to be decomposed into parts that can run independently of each other.

- Concurrency: Different parts of a program execute independently 
- Parallelism: Different parts of a program execute at the same time 
- For the sake of this lecture, we're labeling both as Concurrency

Concurrency enables us to take advantage of multiple processors in computers

Many of the challenges of Concurrency are solved by Ownership -- removes issues with memory safety or data races

# First things first: Threads

In most current operating systems, an executed program's code is run in a processes at once. Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.

Splitting the computation in your program into multiple threads can improve performance because the program does multiple tasks at the same time, but it also adds complexity. Because threads can run simultaneously, there's no inherent guarantee about the order in which parts of your code on different threads will run.

*thread::spawn(**closure containing code we want in another thread**)*