mod player {
   // private function
   fn focus() {
       println!("called player::focus");
   }

   // public function
   pub fn shift() {
       println!("called player::shift");
   }

   // public function
   pub fn jump() {
       // call private function focus and shift inside the module
       focus();
       shift();
       println!("called player::jump");
   }
}

fn main() {
   // call public function jump from player module
   player::jump();
}

/*
url:
https://www.programiz.com/rust/module

Page Title:
Rust Module

Section Title:
Visibility of Items inside a Module in Rust


Subsection Title:
Example: Using Module in Rust


### Documentation

documentation2/B01-Tutorial-module.md


### Compiler Messages




### Compiler error


### Output

called player::focus
called player::shift
called player::jump

### Notes:


*/
