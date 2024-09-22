fn main() {
    let colors = vec!["blue", "red", "green"];
    
    // method 2: access vector elements using get() method and vector index
    println!("first color = {:?}", colors.get(0));
    println!("second color = {:?}", colors.get(1));
    println!("third color = {:?}", colors.get(2));
}

/*
url:
https://www.programiz.com/rust/vector

Page Title:
Rust Vector

Section Title:
Accessing Elements of a Vector using the get() method in Rust

Subsection Title:
Example: Accessing Elements of a Vector using get()

### Documentation

31-Tutorial-Vector-onwards.md

Output:
first color = Some("blue")
second color = Some("red")
third color = Some("green")

Notes:

*/
