fn main() {
    let word = String::from("Hello");

    // immutable closure
    let print_str = || {
        // word variable is moved to a new variable
        let new_word = word;
        println!("word = {}", new_word);
    };

    print_str();

    // cannot immutable borrow because word variable has moved inside closure
    // println!("length of word = {}", word.len());
}

/*
url:
https://www.programiz.com/rust/closure

Page Title:
Rust Closure

Section Title:
Closure Environment Capturing Modes in Rust

Subsection Title:
3. Variable is moved inside closure

### Documentation

30-Tutorial-Closure-onwards.md

Output:
word = Hello

Notes:

*/
