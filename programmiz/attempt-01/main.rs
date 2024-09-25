use std::collections::HashMap;

fn main() {
    let mut fruits: HashMap<i32, String> = HashMap::new();
    
    // add key-value in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));
    
    println!("fruits = {:?}", fruits);
}

/*
url:
https://www.programiz.com/rust/hashmap

Page Title:
Rust HashMap

Section Title:
1. Add Elements to a HashMap in Rust

Subsection Title:
Example: Add Elements to a HashMap

### Documentation

33-Tutorial-HashMap-onwards.md

Output:
fruits = {2: "Banana", 1: "Apple"}

Notes:

*/
