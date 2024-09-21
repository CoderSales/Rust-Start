fn main() {
    let word = String::from("Hello");
    
    // immutable closure
    let print_str = || {
        println!("word = {}", word);
    };

    // immutable borrow is possible outside the closure
    println!("length of word = {}", word.len());
    
    print_str();
}

/*
url:
https://www.programiz.com/rust/closure

Page Title:
Rust Closure

Section Title:
Closure Environment Capturing Modes in Rust

Subsection Title:
1. Variable is not modified inside closure

### Documentation

30-Tutorial-Closure-onwards.md

Output:
length of word = 5
word = Hello

Notes:

*/
