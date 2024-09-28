use std::collections::HashSet;

fn main() {
    let mut colors: HashSet<&str> = HashSet::new();

    colors.insert("Red");
    colors.insert("Yellow");

    println!("colors = {:?}", colors);

    // check for a value in a HashSet
    if colors.contains("Red") {
        println!("We have the color \"Red\" in the HashSet.")
    }
}

/*
url:
https://www.programiz.com/rust/hashset

Page Title:
Rust HashSet

Section Title:
2. Check Value is Present in a HashSet in Rust

Subsection Title:

### Documentation

34-Tutorial-HashSet-onwards.md

Output:
colors = {"Red", "Yellow"}
We have the color "Red" in the HashSet.

Notes:

*/
