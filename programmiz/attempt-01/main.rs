fn main() {
    // scope of outer_var variable is inside the main function code block
    let outer_var = 100;
    
    // start of the inner code block
    {
        // scope of inner_var variable is only inside this new code block
        let inner_var = 200;
        println!("inner_var = {}", inner_var);
    }
    // end of the inner code block
    
    println!("inner_var = {}", inner_var);
    println!("outer_var = {}", outer_var);
}

/*
url:
https://www.programiz.com/rust/variable-scope

Page Title:
Rust Variable Scope

Section Title:
Working of Variable Scope in Rust

Subsection Title:


### Documentation

29-Tutorial-Functions-onwards.md

Output:

Notes:

Error:
error[E0425]: cannot find value `inner_var` in this scope
  --> main.rs:13:32
   |
13 |     println!("inner_var = {}", inner_var);
   |                                ^^^^^^^^^ help: a local variable with a similar name exists: `outer_var`      

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.


*/
