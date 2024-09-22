fn main() {
    let mut even_numbers = vec![2, 4, 6, 8, 10];
    
    println!("original vector = {:?}", even_numbers);
    
    // push values at the end of the vector
    even_numbers.push(12);
    even_numbers.push(14);
    
    println!("changed vector = {:?}", even_numbers);
}

/*
url:
https://www.programiz.com/rust/vector

Page Title:
Rust Vector

Section Title:
Adding Values to a Vector in Rust

Subsection Title:

### Documentation

31-Tutorial-Vector-onwards.md

Output:
original vector = [2, 4, 6, 8, 10]
changed vector = [2, 4, 6, 8, 10, 12, 14]

Notes:

*/
