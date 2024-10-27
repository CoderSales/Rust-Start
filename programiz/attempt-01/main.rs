mod config {
   // items in modules by default have private visibility
   fn select() {
       println!("called config::select");
   }

   // use the `pub` keyword to override private visibility
   pub fn print() {
       println!("called config::print");
   }
}

fn main() {
   // Your program will start here.
   println!("Hello world!");
}

/*
url:
https://www.programiz.com/rust/module

Page Title:
Rust Module

Section Title:
Visibility of Items inside a Module in Rust


Subsection Title:


### Documentation

documentation2/B01-Tutorial-module.md


### Compiler Messages


### Compiler error

error[E0601]: `main` function not found in crate `main`
  --> main.rs:11:2
   |
11 | }
   |  ^ consider adding a `main` function to `main.rs`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0601`.

### more on error from rustc


$ rustc --explain 0601
No `main` function was found in a binary crate.

To fix this error, add a `main` function:

```
fn main() {
    // Your program will start here.
    println!("Hello world!");
}
```

If you don't know the basics of Rust, you can look at the
[Rust Book][rust-book] to get started.

[rust-book]: https://doc.rust-lang.org/book/
-- More  --


### Output


### Notes:


*/
