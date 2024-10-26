fn main() {
    let colors = vec!["Red", "Yellow", "Green"];
    
    // using iter() to iterate through a collection
    for color in colors.iter() {
        // reference to the items in the iterator
        println!("{}", color);
    }
    
    // the collection is untouched and still available here
    println!("colors = {:?}", colors);
}

/*
url:
https://www.programiz.com/rust/iterator

Page Title:
Rust Iterator

Section Title:
Ways to Create Iterator in Rust

Subsection Title:
1. Using iter() method

### Documentation

36-Tutorial-Iterators-onwards.md

Output

Red
Yellow
Green
colors = ["Red", "Yellow", "Green"]

Notes:
Order of values
in Output 
vary
but
the actual
values
themselves 
are the same

*/
