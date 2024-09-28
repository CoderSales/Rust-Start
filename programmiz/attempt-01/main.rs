use std::collections::HashMap;

fn main() {
    let mut fruits: HashMap<i32, String> = HashMap::new();
    
    // insert values in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));
    
    println!("Before update = {:?}", fruits);
    
    // change value of hashmap with key of 1
    fruits.insert(1, String::from("Mango"));
    
    println!("After update = {:?}", fruits);
}

/*
url:
https://www.programiz.com/rust/hashmap

Page Title:
Rust HashMap

Section Title:
4. Change Elements of a HashMap in Rust

Subsection Title:
Example: Change Elements of a HashMap

### Documentation

33-Tutorial-HashMap-onwards.md

Output:
Before update = {2: "Banana", 1: "Apple"}
After update = {2: "Banana", 1: "Mango"}

Notes:

*/
