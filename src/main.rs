// fn main() {
//     println!("Hello, world!");
// }

// fn is basically function here for the defining the working of the program
// main  is entry point of your program and
// the program will start its execution from here

// println! is a macro for printing the output to the console

// cargo.toml is store the metdata of the program and dependencies

// in rust you should have to declare your variable using let

// fn main() {
//     let name = "vishal";
//     let age = 23;
//     println!("name:{}", name);
//     println!("age:{}", age);

//     // let call function here which i have made with name vishal and it solution for Dead_code detection in rust,we can address this using calling function in the main
//     // to not show dead code warning

//     vishal();
// }

// this {} curly brace are used as the placeholders for the variables
// fn vishal() {
//     let language = "Rust";
//     let purpose = "backend development";

//     println!("{language} is useful for {purpose}.");
// }

// Rust has two types of variables
// 1. mutable
// 2. immutable

// important concept in rust
// is Immutability by default

// If you need to change a value, use mut:

// fn main() {
//     let age = 23;
//     age = 24;
//     println!("age is :{}", age);
// }

// fn main() {
//     let mut age = 22;

//     age = 23;

//     println!("{age}");
// }

// data types in rust ->what are the values are using to assigning into the variables is called as data types
//  and every value has its own data types

// code with example using hand

fn main() {
    let username: &str = "vishal";
    let age: i32 = 23;
    let height: f64 = 5.9;
    let is_student: bool = true;
    let salary: f64 = 50000.90;
    println!(
        "username:{} 
        age:{} 
        height:{} 
        is_student:{} 
        salary:{}",
        username, age, height, is_student, salary
    );
}

// // What is snake_case?
// In Rust, variable and function names must be written in snake_case, which means:

// Every letter is lowercase (no capital letters).

// Words are separated by underscores (_).

// Because is_Student has a capital S, the Rust compiler flags it.

// How to Fix It
// Change the capital S to a lowercase s on line 66 of src\main.rs (and anywhere else you used that variable, like inside your println! macro):

// Rust
// // Before (triggers warning)
// let is_Student: bool = true;

// // After (clean)
// let is_student: bool = true;

// > ## Common Rust Data Types
// >
// > | Rust Type | Meaning | Example |
// > |---|---|---|
// > | `i32` | 32-bit signed integer | `-10` |
// > | `u32` | 32-bit unsigned integer | `25` |
// > | `i64` | 64-bit signed integer | `1000` |
// > | `f64` | 64-bit floating-point number | `99.5` |
// > | `bool` | Boolean value | `true` |
// > | `&str` | String slice | `"Hello"` |
// > | `String` | Owned, growable string | `String::from("Hello")` |
// >
// > ### Type Inference
// >
// > You don't always need to specify types explicitly because Rust can often infer them automatically:
// >
// > ```rust
// > let age = 22;       // inferred as i32
// > let price = 99.5;   // inferred as f64
// > let active = true;  // inferred as bool
// > ```
