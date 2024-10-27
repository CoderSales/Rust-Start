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
   // private items inside module cannot be accessed outside the parent module
   // calling private select function inside config module will cause a compilation error
   display::select();
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

error[E0433]: failed to resolve: use of undeclared crate or module `display`
  --> main.rs:16:4
   |
16 |    display::select();
   |    ^^^^^^^ use of undeclared crate or module `display`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0433`.

### Output


### Notes:


*/
