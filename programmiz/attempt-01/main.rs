fn main() {
    // an array of numbers
    let numbers = [1, 2, 3, 4, 5];
    
    // create a slice of 2nd and 3rd element
    let slice = &numbers[1..3];
    
    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);
}

/*
url:
https://www.programiz.com/rust/slice

Page Title:
Rust Slice

Section Title:
Example: Rust Slice

Subsection Title:


Output:
array = [1, 2, 3, 4, 5]
slice = [2, 3]

*/
