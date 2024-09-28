use std::collections::HashMap;

fn main() {
    let mut fruits: HashMap<i32, String> = HashMap::new();
    
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));
    
    // loop and print values of hashmap using values() method
    for fruit in fruits.values() {
        println!("{}", fruit)
    }
    
    // print the length of hashmap using len() method
    println!("Length of fruits = {}", fruits.len());
}

/*
url:
https://www.programiz.com/rust/hashmap

Page Title:
Rust HashMap

Section Title:
Other Methods of Rust HashMap

Subsection Title:
Example: Rust HashMap Methods

### Documentation

33-Tutorial-HashMap-onwards.md

Output:
Apple
Banana
Length of fruits = 2

Notes:

*/
