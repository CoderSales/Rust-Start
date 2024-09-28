use std::collections::HashSet;

fn main() {
    // Create HashSet with default set of values using from() method
    let numbers = HashSet::from([2, 7, 8, 10]);
    
    println!("numbers = {:?}", numbers);
}

/*
url:
https://www.programiz.com/rust/hashset

Page Title:
Rust HashSet

Section Title:
HashSet with Default Values in Rust

Subsection Title:

### Documentation

34-Tutorial-HashSet-onwards.md

Output:
numbers = {2, 7, 8, 10}

Notes:
order of
values
in Output
varies with
each run

*/
