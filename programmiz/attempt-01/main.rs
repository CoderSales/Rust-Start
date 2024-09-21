fn main() {
    let mut age = 100;

    {
        // shadowing by immutable age variable
        let age = age;

        println!("age variable inner block = {}", age);
        // age goes out of scope
    } // Note Key line of code

    // age variable is not frozen in this scope
    age = 3;

    println!("age variable outer block = {}", age);
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
age variable inner block = 100
age variable outer block = 3

Notes:
We can freeze a variable in Rust 
by using shadowing and immutability. 

Once a variable is frozen, 
we cannot change the variable value 
in the inner scope.


Fix


*/
