fn main() {
    let num = 100;
    
    // A closure that captures the num variable
    let print_num = || println!("Number = {}", num);
    
    print_num(); 
}

/*
url:
https://www.programiz.com/rust/closure

Page Title:
Rust Closure

Section Title:
Closure Environment Capturing in Rust

Subsection Title:


### Documentation

30-Tutorial-Closure-onwards.md

Output:
Number = 100

Notes:

*/
