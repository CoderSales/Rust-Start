// nested module
pub mod player {
   pub mod sprite {
       pub fn create() {
           println!("called player::sprite::create");
       }
   }
}

fn main() {
   // call public function create from sprite module which is inside player module 
   player::sprite::create();
}

/*
url:
https://www.programiz.com/rust/module

Page Title:
Rust Module

Section Title:
Nested Modules


Subsection Title:



### Documentation

documentation2/B01-Tutorial-module.md


### Compiler Messages




### Compiler error


### Output

called player::sprite::create


### Notes:


*/
