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
    
    // access value of name field in Person struct
    println!("Person name = {}", person.name);

    // access value of age field in Person struct
    println!("Person age = {}", person.age);

    // access value of height field in Person struct
    println!("Person height = {}", person.height);
}

/*
url:
https://www.programiz.com/rust/struct

Page Title:
Rust Struct

Section Title:


Subsection Title:


### Documentation

28-Tutorial-Tuple-onwards.md

Output:
Person name = John Doe
Person age = 18
Person height = 178

Notes:

*/
