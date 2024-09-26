use std::collections::HashMap;

fn main() {
    let mut fruits: HashMap<i32, String> = HashMap::new();
    
    // insert values in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));
    
    println!("fruits before remove operation = {:?}", fruits);

    // remove value in a hashmap
    fruits.remove(&1);
    
    println!("fruits after remove operation = {:?}", fruits);
}

/*
url:
https://www.programiz.com/rust/hashmap

Page Title:
Rust HashMap

Section Title:
3. Remove Elements from a HashMap in Rust

Subsection Title:
Example: Remove Elements in a HashMap

### Documentation

33-Tutorial-HashMap-onwards.md

Output:
fruits before remove operation = {2: "Banana", 1: "Apple"}
fruits after remove operation = {2: "Banana"}

Notes:

*/
