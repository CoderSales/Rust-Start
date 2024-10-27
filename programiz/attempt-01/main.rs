// nested module
pub mod player {
   pub mod sprite {
       pub fn create() {
           println!("called player::sprite::create");
       }
   }
}

// bring the create function into scope
use player::sprite::create;

fn main() {
   // call public function directly
   create();
}

/*
url:
https://www.programiz.com/rust/module

Page Title:
Rust Module

Section Title:
The use keyword in Rust


Subsection Title:



### Documentation

documentation2/B01-Tutorial-module.md


### Compiler Messages




### Compiler error


### Output

called player::sprite::create

### Notes:


*/
