fn main() {
    // define a Person struct
    struct Person {
        name: String,
        age: u8,
        height: u8
    }
    
    // instantiate Person struct
    let person = Person {
        name: String::from("John Doe"),
        age: 18,
        height: 178
    };
    
    // destructure Person struct into name, age and height variables
    let Person { name, age, height } = person;
    
    println!("Person name = {}", name);
    println!("Person age = {}", age);
    println!("Person height = {}", height);
}

/*
url:
https://www.programiz.com/rust/struct

Page Title:
Rust Struct

Section Title:
Destructuring Fields of a Rust Struct

Subsection Title:
Example: Destructuring Fields of Struct


### Documentation

28-Tutorial-Tuple-onwards.md

Output:
Person name = John Doe
Person age = 18
Person height = 178

Notes:

*/
