fn main() {
    let colors = vec!["Red", "Yellow", "Green"];
    
    // iterator
    let mut colors_iterator = colors.iter();
    println!("colors iterator = {:?}", colors_iterator);
    
    // fetch values from iterator one by one using next() method
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
}

/*
url:
https://www.programiz.com/rust/iterator

Page Title:
Rust Iterator

Section Title:
next() Method of an Iterator in Rust

Subsection Title:

### Documentation

36-Tutorial-Iterators-onwards.md

Output

colors iterator = Iter(["Red", "Yellow", "Green"])
Some("Red")
Some("Yellow")
Some("Green")
None

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
