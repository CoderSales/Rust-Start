use std::collections::HashSet;

fn main() {
    let mut colors: HashSet<&str> = HashSet::new();
    
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");

    // iterate over a hashset
    for color in colors {
        // print each value in the hashset
        println!("{}", color);
    }
}

/*
url:
https://www.programiz.com/rust/hashset

Page Title:
Rust HashSet

Section Title:
4. Iterate over Values of a HashSet in Rust

Subsection Title:

### Documentation

34-Tutorial-HashSet-onwards.md

Output:
Yellow
Green
Red

Notes:
Order of colors
in Output
varies
with each
run

*/
