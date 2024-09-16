fn main() {
    // an array without data type
    let a = [5, 4, 3, 2, 1];
    
    // an array with data type and size
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    
    // an array with default values
    let c = [3; 5];
    
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}

/*
url:
https://www.programiz.com/rust/array

Page Title:
Rust Array

Section Title:
Creating an Array in Rust

Subsection Title:
Revision: Different Ways to Create Array in Rust

Output:
a = [5, 4, 3, 2, 1]
b = [1, 2, 3, 4, 5]
c = [3, 3, 3, 3, 3]

*/
