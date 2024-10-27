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
   // public items inside module can be accessed outside the parent module
   // call public print function from display module
   config::print();
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

warning: 1 warning emitted


### Compiler error



### Output

called config::print

### Notes:


*/
