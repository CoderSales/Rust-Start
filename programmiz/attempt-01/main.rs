// import HashSet from Rust standard collections library
use std::collections::HashSet;

fn main() {
    // create a new HashSet
    let mut color: HashSet<String> = HashSet::new();
    
    println!("HashSet = {:?}", color);
}

/*
url:
https://www.programiz.com/rust/hashset

Page Title:
Rust HashSet

Section Title:
Creating a HashSet in Rust

Subsection Title:
Example: Creating a HashSet

### Documentation

34-Tutorial-HashSet-onwards.md

Compiler Warning:

warning: variable does not need to be mutable
 --> ./main.rs:6:9
  |
6 |     let mut color: HashSet<String> = HashSet::new();
  |         ----^^^^^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

warning: 1 warning emitted

Output:
HashSet = {}

Notes:

*/
