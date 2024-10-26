fn main() {
    let mut colors = vec!["Red", "Yellow", "Green"];
    
    // using iter_mut() to iterate through a collection
    for color in colors.iter_mut() {
        // modify the item in the collection
        *color = "Black";
        println!("{}", color);
    }
    
    // the modified collection is available here
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
3. Using iter_mut() method

### Documentation

36-Tutorial-Iterators-onwards.md

Output

Black
Black
Black
colors = ["Black", "Black", "Black"]

Notes:

*/
