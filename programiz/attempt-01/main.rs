fn main() {
    let text = "Hello, World!";
    
    let character_option = text.chars().nth(11);
    
    // using match for Option type
    let character = match character_option {
        None => "empty".to_string(),
        Some(c) => c.to_string()
    };
    
    println!("Character at index 11 is {}", character);
}

/*
url:
https://www.programiz.com/rust/error-handling

Page Title:
Rust Error Handling

Section Title:
The Option Enum

Subsection Title:
Part 2


### Documentation

37-Error-Handling.md


### Compiler Messages


### Output

Character at index 11 is d


### Notes:

*/
