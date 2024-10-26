fn main() {
    // number comes into scope
    let number = 10;
    
    // value of the number is copied into the function
    print_number(number);
    
    // number variable can be used here
    println!("number = {}", number);
}

fn print_number(value: i32) { // value comes into scope
    println!("value = {}", value);
}   // value goes out of scope

/*
url:
https://www.programiz.com/rust/ownership

Page Title:
Ownership

Section Title:
Ownership in Functions


Subsection Title:


### Documentation

39-Ownership.md


### Compiler Messages


### Output

value = 10
number = 10

### Notes:


*/
