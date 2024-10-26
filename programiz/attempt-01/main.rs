fn main() {
    println!("Hello, World!");

    // Explicitly exit the program with an unrecoverable error
    panic!("Crash");
}

/*
url:
https://www.programiz.com/rust/error-handling

Page Title:
Rust Error Handling

Section Title:
Unrecoverable Errors in Rust

Subsection Title:


### Documentation

36-Tutorial-Iterators-onwards.md

Output

Hello, World!
thread 'main' panicked at main.rs:5:5:
Crash
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

Notes:

*/
