fn main() {
    let my_option: Option<i32> = Some(222);
    
    // use of match expression to match Option type
    match my_option {
        Some(value) => println!("The option has a value of {}", value),
        None => println!("The option has no value"),
    }
}

/*
url:
https://www.programiz.com/rust/pattern-matching

Page Title:
Rust Pattern Matching


Section Title:
Matching Option and Result Type in Rust

Subsection Title:
Example: Matching Option Type in Rust


Output:
The option has a value of 222

*/
