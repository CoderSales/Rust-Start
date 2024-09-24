// import HashMap from Rust standard collections library
use std::collections::HashMap;

fn main() {
    // create a new HashMap
    let mut info: HashMap<i32, String> = HashMap::new();
    
    println!("HashMap = {:?}", info);
}

/*
url:
https://www.programiz.com/rust/hashmap

Page Title:
Rust HashMap

Section Title:
Creating a HashMap in Rust

Subsection Title:
Example: Creating a HashMap

### Documentation

33-Tutorial-HashMap-onwards.md

Compiler Warning:

warning: variable does not need to be mutable
 --> ./main.rs:6:9
  |
6 |     let mut info: HashMap<i32, String> = HashMap::new();
  |         ----^^^^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

warning: 1 warning emitted

Output:
HashMap = {}

Notes:

*/
