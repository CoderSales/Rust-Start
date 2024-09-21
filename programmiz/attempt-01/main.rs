fn main() {
    let mut word = String::from("Hello");
    
    // mutable closure
    let mut print_str = || {
        // value of word is changed here
        word.push_str(" World!");
        println!("word = {}", word);
    };
     
     // cannot immutable borrow because the variable is borrowed as mutable inside the closure
     // println!("length of word = {}", word.len());
    
    print_str();

    // can immutable borrow because the closure has been already used
    println!("length of word = {}", word.len());
}

/*
url:
https://www.programiz.com/rust/closure

Page Title:
Rust Closure

Section Title:
Closure Environment Capturing Modes in Rust

Subsection Title:
2. Variable is modified inside closure

### Documentation

30-Tutorial-Closure-onwards.md

Output:
word = Hello World!
length of word = 12

Notes:

*/
