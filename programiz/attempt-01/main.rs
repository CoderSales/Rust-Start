fn main() {
    let colors = vec!["Red", "Yellow", "Green"];
    
    // using into_iter() to iterate through a collection
    for color in colors.into_iter() {
        // the items in the collection move into this scope
        println!("{}", color);
    }
    // end of scope of for loop
    
    // error
    // the collection is not available here as the for loop scope ends above
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
2. Using into_iter() method

### Documentation

36-Tutorial-Iterators-onwards.md

Output


Error

error[E0382]: borrow of moved value: `colors`
  --> main.rs:13:31
   |
2  |     let colors = vec!["Red", "Yellow", "Green"];
   |         ------ move occurs because `colors` has type `Vec<&str>`, which does not implement the `Copy` trait 
...
5  |     for color in colors.into_iter() {
   |                         ----------- `colors` moved due to this method call
...
13 |     println!("colors = {:?}", colors);
   |                               ^^^^^^ value borrowed here after move
   |
note: `into_iter` takes ownership of the receiver `self`, which moves `colors`
  --> /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14\library\core\src\iter\traits\collect.rs:346:18
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: you can `clone` the value and consume it, but this might not be your desired behavior
   |
5  |     for color in colors.clone().into_iter() {
   |                        ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.

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
