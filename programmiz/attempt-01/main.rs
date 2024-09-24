fn main() {
    // create an empty string
    let mut word = String::new();
    
    println!("original string = {}", word);
    
    // append a string to the word variable
    word.push_str("Hello, World!");

    println!("changed string = {}", word);
}

/*
url:
https://www.programiz.com/rust/string

Page Title:
Rust String

Section Title:
Creating an Empty String with String::new()

Subsection Title:
Example: Creating an Empty String with String::new()

### Documentation

32-Tutorial-Rust-String-onwards.md

Output:
original string = 
changed string = Hello, World!

Notes:

*/
