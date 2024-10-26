fn main() {
    // owner of the String value
    // rule no. 1 
    let fruit1 = String::from("Banana");
    
    // ownership moves to another variable
    // only one owner at a time
    // rule no. 2
    let fruit2 = fruit1;
    
    // cannot print variable fruit1 because ownership has moved
    // error, out of scope, value is dropped
    // rule no. 3
    // println!("fruit1 = {}", fruit1);
    
    // print value of fruit2 on the screen
    println!("fruit2 = {}", fruit2);
}

/*
url:
https://www.programiz.com/rust/unwrap-and-expect

Page Title:
Ownership

Section Title:

Subsection Title:


### Documentation

39-Ownership.md


### Compiler Messages


### Output
fruit2 = Banana

### Notes:

*/
