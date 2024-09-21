fn main() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
    println!("original array = {:?}", numbers);
    
    // change the value of the 3rd element in the array
    numbers[2] = 0;
    
    println!("changed array = {:?}", numbers);
}

/*
url:
https://www.programiz.com/rust/array

Page Title:
Rust Array

Section Title:
Mutable Array in Rust

Subsection Title:


Output:
original array = [1, 2, 3, 4, 5]
changed array = [1, 2, 0, 4, 5]

*/
