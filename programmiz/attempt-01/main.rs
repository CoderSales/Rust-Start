fn main() {
    let mut word = String::from("cat");
    
    println!("original string = {}", word);
    
    // push a new string at the end of the initial string 
    word.push_str(" dog");
    
    println!("changed string = {}", word);
}

/*
url:
https://www.programiz.com/rust/string

Page Title:
Rust String

Section Title:
Mutable String in Rust

Subsection Title:

### Documentation

32-Tutorial-Rust-String-onwards.md

Output:
original string = cat
changed string = cat dog

Notes:

*/
