fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3];
    
    // using the map iterator adapter
    let even_numbers: Vec<i32> = numbers.iter().map(|i| i * 2).collect();
    
    println!("numbers = {:?}", numbers);
    println!("even_numbers = {:?}", even_numbers);
 }
 
/*
url:
https://www.programiz.com/rust/iterator

Page Title:
Rust Iterator

Section Title:
### Example: Iterator Adapters

Subsection Title:


### Documentation

36-Tutorial-Iterators-onwards.md

Output
numbers = [1, 2, 3]
even_numbers = [2, 4, 6]

Notes:

*/
