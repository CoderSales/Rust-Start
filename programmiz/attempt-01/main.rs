fn main() {
    let numbers = [1, 2, 3, 4, 5];
    
    // omit the start index and the end index
    // reference the whole array
    let slice = &numbers[..];

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
3. Omitting both Start and End Index of a Slice


Output:
array = [1, 2, 3, 4, 5]
slice = [1, 2, 3, 4, 5]

*/
