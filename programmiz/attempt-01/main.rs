fn main() {
    // mutable array
    let mut colors = ["red", "green", "yellow", "white"];
    
    println!("array = {:?}", colors);

    // mutable slice
    let sliced_colors = &mut colors[1..3]; // Note
    
    println!("original slice = {:?}", sliced_colors);

    // change the value of the original slice at the first index
    sliced_colors[1] = "purple";

    println!("changed slice = {:?}", sliced_colors);
}

/*
url:
https://www.programiz.com/rust/slice

Page Title:
Rust Slice

Section Title:
Mutable Slice in Rust

Subsection Title:


Output:
array = ["red", "green", "yellow", "white"]
original slice = ["green", "yellow"]
changed slice = ["green", "purple"]

Note:
Mutable Slice

*/
