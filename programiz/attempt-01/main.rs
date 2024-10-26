fn main() {
    let numbers = [1, 2 ,3];

    println!("unknown index value = {}", numbers[3]);
}

/*
url:
https://www.programiz.com/rust/error-handling

Page Title:
Rust Error Handling

Section Title:
Unrecoverable Errors in Rust

Subsection Title:
Example 2: Rust Unrecoverable Errors

### Documentation

36-Tutorial-Iterators-onwards.md

Output

error: this operation will panic at runtime
 --> main.rs:4:42
  |
4 |     println!("unknown index value = {}", numbers[3]);
  |                                          ^^^^^^^^^^ index out of bounds: the length is 3 but the index is 3  
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: aborting due to 1 previous error


Notes:

*/
