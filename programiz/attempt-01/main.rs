use std::collections::HashSet;

fn main() {
    let mut colors: HashSet<&str> = HashSet::new();

    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");

    println!("colors before remove operation = {:?}", colors);

    // remove value from a HashSet
    colors.remove("Yellow");
    
    println!("colors after remove operation = {:?}", colors);
}

/*
url:
https://www.programiz.com/rust/hashset

Page Title:
Rust HashSet

Section Title:
3. Remove Values from a HashSet in Rust

Subsection Title:

### Documentation

34-Tutorial-HashSet-onwards.md

Output:
colors before remove operation = {"Yellow", "Red", "Green"}
colors after remove operation = {"Red", "Green"}

Notes:

*/
