fn main() {
    // create a new String.
    let fruit1 = String::from("Banana");
    
    // create a copy of fruit1 using the clone method.
    let fruit2 = fruit1.clone();
    

    println!("fruit1 = {}", fruit1);
    
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
fruit1 = Banana
fruit2 = Banana

### Notes:
Note: Using clone() can incur additional runtime cost, so it should be used sensibly.


*/
