fn main() {
    let my_result: Result<i32, &str> = Ok(100);

    // use of match expression to match Result type
    match my_result {
        Ok(value) => println!("The result is {}", value),
        Err(error) => println!("The error message is {}", error),
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

(Second example)


Output:

The result is 100

*/
