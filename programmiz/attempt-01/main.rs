fn main() {
    let my_option: Option<i32> = Some(222);

    // use of if let expression on the Option type
    if let Some(value) = my_option {
        println!("The option has a value of {}", value);
    } else {
        println!("The option has no value");
    }
}

/*
url:
https://www.programiz.com/rust/pattern-matching

Page Title:
Rust Pattern Matching


Section Title:
if let Expression in Rust

Subsection Title:

Output:

The option has a value of 222

*/
