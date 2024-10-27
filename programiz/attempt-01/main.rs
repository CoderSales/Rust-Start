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

warning: function `select` is never used
 --> main.rs:3:7
  |
3 |    fn select() {
  |       ^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: function `print` is never used
 --> main.rs:8:11
  |
8 |    pub fn print() {
  |           ^^^^^

warning: 2 warnings emitted

### Compiler error



### Output

Hello world!

### Notes:


*/
