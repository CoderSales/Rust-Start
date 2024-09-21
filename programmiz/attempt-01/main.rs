fn main() {
    let mixture = ("Hello, World!", 16, 2.71828);
    
    // destructuring a tuple
    let (message, number, float) = mixture;
    
    println!("message = {}", message);
    println!("number = {}", number);
    println!("float = {}", float);
}

/*
url:
https://www.programiz.com/rust/tuple

Page Title:
Rust Tuple

Section Title:
Destructuring a Tuple

Subsection Title:
Example: Destructuring a Tuple

### Documentation

28-Tutorial-Tuple-onwards.md

Output:
message = Hello, World!
number = 16
float = 2.71828

Notes:

*/
