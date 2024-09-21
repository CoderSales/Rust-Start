fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // omit the end index
    let slice = &numbers[2..];

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
2. Omitting the End Index of a Slice

Output:
array = [1, 2, 3, 4, 5]
slice = [3, 4, 5]

*/
