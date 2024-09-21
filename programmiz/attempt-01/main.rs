fn main() {
    let random = 100;

    // start of the inner block
    {
        println!("random variable before shadowing in inner block = {}", random);

        // this declaration shadows the outer random variable
        let random = "abc";

        println!("random after shadowing in inner block = {}", random);
    }
    // end of the inner block

    println!("random variable in outer block = {}", random);
}

/*
url:
https://www.programiz.com/rust/variable-scope

Page Title:
Rust Variable Scope

Section Title:
Variable Shadowing in Rust

Subsection Title:


### Documentation

29-Tutorial-Functions-onwards.md

Output:
random variable before shadowing in inner block = 100
random after shadowing in inner block = abc
random variable in outer block = 100

Notes:


*/
