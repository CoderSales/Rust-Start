fn main() {
    // define a multi-line closure
    let squared_sum = |x: i32, y: i32| {
    
        // find the sum of two parameters
        let mut sum: i32 = x + y;
        
        // find the squared value of the sum
        let mut result: i32 = sum * sum;
        
        return result;
    };
    
    // call the closure
    let result = squared_sum(5, 3);
    
    println!("Result = {}", result);
}

/*
url:
https://www.programiz.com/rust/closure

Page Title:
Rust Closure

Section Title:
Multi-line Closure in Rust

Subsection Title:


### Documentation

30-Tutorial-Closure-onwards.md

Output:
Result = 64

Notes:
2 warnings about
2 variables
that do not need to be mutable

Warnings:

warning: variable does not need to be mutable
 --> main.rs:6:13
  |
6 |         let mut sum: i32 = x + y;
  |             ----^^^
  |             |
  |             help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
 --> main.rs:9:13
  |
9 |         let mut result: i32 = sum * sum;
  |             ----^^^^^^
  |             |
  |             help: remove this `mut`

warning: 2 warnings emitted

*/
