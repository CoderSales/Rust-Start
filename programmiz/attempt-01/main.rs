fn main() {
    let colors = vec!["blue", "red", "green"];
    
    let mut index = 0;
    
    // loop through a vector to print its index and value
    for color in colors {
        println!("Index: {} -- Value: {}", index, color);
        index = index + 1;
    }
}

/*
url:
https://www.programiz.com/rust/vector

Page Title:
Rust Vector

Section Title:
Looping Through a Vector in Rust

Subsection Title:

### Documentation

31-Tutorial-Vector-onwards.md

Output:
Index: 0 -- Value: blue
Index: 1 -- Value: red
Index: 2 -- Value: green

Notes:

*/
