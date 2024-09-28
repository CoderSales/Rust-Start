use std::collections::HashSet;

fn main() {
    let mut colors: HashSet<&str> = HashSet::new();
    
    // insert values in a HashSet
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");

    println!("colors = {:?}", colors);
}

/*
url:
https://www.programiz.com/rust/hashset

Page Title:
Rust HashSet

Section Title:
1. Add Values to a HashSet in Rust

Subsection Title:
Example: Add Values to a HashSet

### Documentation

34-Tutorial-HashSet-onwards.md

Output:
colors = {"Yellow", "Red", "Green"}

Notes:

*/
