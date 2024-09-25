use std::collections::HashMap;

fn main() {
    let mut fruits: HashMap<i32, String> = HashMap::new();
    
    // insert elements in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));
    
    // access values in a hashmap
    let first_fruit = fruits.get(&1);
    let second_fruit = fruits.get(&2);
    let third_fruit = fruits.get(&3);
    
    println!("first fruit = {:?}", first_fruit);
    println!("second fruit = {:?}", second_fruit);
    println!("third fruit = {:?}", third_fruit);
}

/*
url:
https://www.programiz.com/rust/hashmap

Page Title:
Rust HashMap

Section Title:
2. Access Values in a HashMap in Rust

Subsection Title:
Example: Access Values in a HashMap

### Documentation

33-Tutorial-HashMap-onwards.md

Output:
first fruit = Some("Apple")
second fruit = Some("Banana")
third fruit = None

Notes:

*/
