fn main() {
    let str = String::from("Hello, World!");
    
    // Call function with reference String value
    let len = calculate_length(&str);

    println!("The length of '{}' is {}.", str, len);
}

// Function to calculate length of a string
// It takes a reference of a String as an argument
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}


/*
url:
https://www.programiz.com/rust/ownership

Page Title:
Rust References and Borrowing

Section Title:
Understanding References in Rust


Subsection Title:


### Documentation

40-Tutorail-References-and-Borrowing.md


### Compiler Messages


### Output

The length of 'Hello, World!' is 13.

### Notes:


*/
