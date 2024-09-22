fn main() {
    let mut even_numbers = vec![2, 4, 6, 8, 10];
    
    println!("original vector = {:?}", even_numbers);
    
    // remove value from the vector in its second index
    even_numbers.remove(2);
    
    println!("changed vector = {:?}", even_numbers);
}

/*
url:
https://www.programiz.com/rust/vector

Page Title:
Rust Vector

Section Title:
Removing Values from a Vector in Rust

Subsection Title:

### Documentation

31-Tutorial-Vector-onwards.md

Output:
original vector = [2, 4, 6, 8, 10]
changed vector = [2, 4, 8, 10]

Notes:

*/
