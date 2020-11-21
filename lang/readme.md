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