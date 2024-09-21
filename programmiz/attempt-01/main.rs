fn main() {
    let mut age = 1;

    // start of the inner block
    {
        // shadowing by immutable age variable
        let age = age;

        // error, age variable is frozen in this scope
        age = 2;

        println!("age variable inner block = {}", age);
        // age variable goes out of scope
    }
    // end of the inner block

    // age variable is not frozen in outer block
    age = 3;

    println!("integer variable outer block = {}", age);
}

/*
url:
https://www.programiz.com/rust/variable-scope

Page Title:
Rust Variable Scope

Section Title:
Variable Freezing in Rust

Subsection Title:


### Documentation

29-Tutorial-Functions-onwards.md

Output:

Notes:
We can freeze a variable in Rust 
by using shadowing and immutability. 

Once a variable is frozen, 
we cannot change the variable value 
in the inner scope.



Error and warning:

warning: value assigned to `age` is never read
 --> main.rs:7:13
  |
7 |         let age = age;
  |             ^^^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

error[E0384]: cannot assign twice to immutable variable `age`
  --> main.rs:10:9
   |
7  |         let age = age;
   |             ---
   |             |
   |             first assignment to `age`
   |             help: consider making this binding mutable: `mut age`
...
10 |         age = 2;
   |         ^^^^^^^ cannot assign twice to immutable variable

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0384`.

*/
