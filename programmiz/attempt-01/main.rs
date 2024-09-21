fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // omit the start index
    let slice = &numbers[..3];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);
}

/*
url:
https://www.programiz.com/rust/slice

Page Title:
Rust Slice

Section Title:
Omit Indexes of a Rust Slice

Subsection Title:
1. Omitting the Start Index of a Slice

Output:
array = [1, 2, 3, 4, 5]
slice = [1, 2, 3]

*/
